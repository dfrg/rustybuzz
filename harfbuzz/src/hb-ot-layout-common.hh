/*
 * Copyright © 2007,2008,2009  Red Hat, Inc.
 * Copyright © 2010,2012  Google, Inc.
 *
 *  This is part of HarfBuzz, a text shaping library.
 *
 * Permission is hereby granted, without written agreement and without
 * license or royalty fees, to use, copy, modify, and distribute this
 * software and its documentation for any purpose, provided that the
 * above copyright notice and the following two paragraphs appear in
 * all copies of this software.
 *
 * IN NO EVENT SHALL THE COPYRIGHT HOLDER BE LIABLE TO ANY PARTY FOR
 * DIRECT, INDIRECT, SPECIAL, INCIDENTAL, OR CONSEQUENTIAL DAMAGES
 * ARISING OUT OF THE USE OF THIS SOFTWARE AND ITS DOCUMENTATION, EVEN
 * IF THE COPYRIGHT HOLDER HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 *
 * THE COPYRIGHT HOLDER SPECIFICALLY DISCLAIMS ANY WARRANTIES, INCLUDING,
 * BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
 * FITNESS FOR A PARTICULAR PURPOSE.  THE SOFTWARE PROVIDED HEREUNDER IS
 * ON AN "AS IS" BASIS, AND THE COPYRIGHT HOLDER HAS NO OBLIGATION TO
 * PROVIDE MAINTENANCE, SUPPORT, UPDATES, ENHANCEMENTS, OR MODIFICATIONS.
 *
 * Red Hat Author(s): Behdad Esfahbod
 * Google Author(s): Behdad Esfahbod
 */

#ifndef HB_OT_LAYOUT_COMMON_HH
#define HB_OT_LAYOUT_COMMON_HH

#include "hb.hh"
#include "hb-ot-layout.hh"
#include "hb-open-type.hh"
#include "hb-set.hh"
#include "hb-map.hh"

#ifndef HB_MAX_NESTING_LEVEL
#define HB_MAX_NESTING_LEVEL 6
#endif
#ifndef HB_MAX_CONTEXT_LENGTH
#define HB_MAX_CONTEXT_LENGTH 64
#endif
#ifndef HB_CLOSURE_MAX_STAGES
/*
 * The maximum number of times a lookup can be applied during shaping.
 * Used to limit the number of iterations of the closure algorithm.
 * This must be larger than the number of times add_pause() is
 * called in a collect_features call of any shaper.
 */
#define HB_CLOSURE_MAX_STAGES 32
#endif

#ifndef HB_MAX_SCRIPTS
#define HB_MAX_SCRIPTS 500
#endif

#ifndef HB_MAX_LANGSYS
#define HB_MAX_LANGSYS 2000
#endif

#ifndef HB_MAX_FEATURES
#define HB_MAX_FEATURES 750
#endif

#ifndef HB_MAX_FEATURE_INDICES
#define HB_MAX_FEATURE_INDICES 1500
#endif

#ifndef HB_MAX_LOOKUP_INDICES
#define HB_MAX_LOOKUP_INDICES 20000
#endif

namespace OT {

#define NOT_COVERED ((unsigned int)-1)

struct hb_collect_variation_indices_context_t : hb_dispatch_context_t<hb_collect_variation_indices_context_t>
{
    template <typename T> return_t dispatch(const T &obj)
    {
        obj.collect_variation_indices(this);
        return hb_empty_t();
    }
    static return_t default_return_value()
    {
        return hb_empty_t();
    }

    hb_set_t *layout_variation_indices;
    const hb_set_t *glyph_set;
    const hb_map_t *gpos_lookups;

    hb_collect_variation_indices_context_t(hb_set_t *layout_variation_indices_,
                                           const hb_set_t *glyph_set_,
                                           const hb_map_t *gpos_lookups_)
        : layout_variation_indices(layout_variation_indices_)
        , glyph_set(glyph_set_)
        , gpos_lookups(gpos_lookups_)
    {
    }
};

/*
 *
 * OpenType Layout Common Table Formats
 *
 */

/*
 * Script, ScriptList, LangSys, Feature, FeatureList, Lookup, LookupList
 */

struct Record_sanitize_closure_t
{
    hb_tag_t tag;
    const void *list_base;
};

template <typename Type> struct Record
{
    int cmp(hb_tag_t a) const
    {
        return tag.cmp(a);
    }

    bool sanitize(hb_sanitize_context_t *c, const void *base) const
    {
        TRACE_SANITIZE(this);
        const Record_sanitize_closure_t closure = {tag, base};
        return_trace(c->check_struct(this) && offset.sanitize(c, base, &closure));
    }

    Tag tag;               /* 4-byte Tag identifier */
    OffsetTo<Type> offset; /* Offset from beginning of object holding
                            * the Record */
public:
    DEFINE_SIZE_STATIC(6);
};

template <typename Type> struct RecordArrayOf : SortedArrayOf<Record<Type>>
{
    const OffsetTo<Type> &get_offset(unsigned int i) const
    {
        return (*this)[i].offset;
    }
    OffsetTo<Type> &get_offset(unsigned int i)
    {
        return (*this)[i].offset;
    }
    const Tag &get_tag(unsigned int i) const
    {
        return (*this)[i].tag;
    }
    unsigned int
    get_tags(unsigned int start_offset, unsigned int *record_count /* IN/OUT */, hb_tag_t *record_tags /* OUT */) const
    {
        if (record_count) {
            +this->sub_array(start_offset, record_count) | hb_map(&Record<Type>::tag) |
                hb_sink(hb_array(record_tags, *record_count));
        }
        return this->len;
    }
    bool find_index(hb_tag_t tag, unsigned int *index) const
    {
        return this->bfind(tag, index, HB_BFIND_NOT_FOUND_STORE, Index::NOT_FOUND_INDEX);
    }
};

template <typename Type> struct RecordListOf : RecordArrayOf<Type>
{
    const Type &operator[](unsigned int i) const
    {
        return this + this->get_offset(i);
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        return_trace(RecordArrayOf<Type>::sanitize(c, this));
    }
};

struct Feature;

struct RecordListOfFeature : RecordListOf<Feature>
{
};

struct RangeRecord
{
    int cmp(hb_codepoint_t g) const
    {
        return g < first ? -1 : g <= last ? 0 : +1;
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        return_trace(c->check_struct(this));
    }

    bool intersects(const hb_set_t *glyphs) const
    {
        return glyphs->intersects(first, last);
    }

    template <typename set_t> bool collect_coverage(set_t *glyphs) const
    {
        return glyphs->add_range(first, last);
    }

    HBGlyphID first; /* First GlyphID in the range */
    HBGlyphID last;  /* Last GlyphID in the range */
    HBUINT16 value;  /* Value */
public:
    DEFINE_SIZE_STATIC(6);
};
DECLARE_NULL_NAMESPACE_BYTES(OT, RangeRecord);

struct IndexArray : ArrayOf<Index>
{
    bool intersects(const hb_map_t *indexes) const
    {
        return hb_any(*this, indexes);
    }

    unsigned int
    get_indexes(unsigned int start_offset, unsigned int *_count /* IN/OUT */, unsigned int *_indexes /* OUT */) const
    {
        if (_count) {
            +this->sub_array(start_offset, _count) | hb_sink(hb_array(_indexes, *_count));
        }
        return this->len;
    }

    void add_indexes_to(hb_set_t *output /* OUT */) const
    {
        output->add_array(arrayZ, len);
    }
};

struct LangSys
{
    unsigned int get_feature_count() const
    {
        return featureIndex.len;
    }
    hb_tag_t get_feature_index(unsigned int i) const
    {
        return featureIndex[i];
    }
    unsigned int get_feature_indexes(unsigned int start_offset,
                                     unsigned int *feature_count /* IN/OUT */,
                                     unsigned int *feature_indexes /* OUT */) const
    {
        return featureIndex.get_indexes(start_offset, feature_count, feature_indexes);
    }
    void add_feature_indexes_to(hb_set_t *feature_indexes) const
    {
        featureIndex.add_indexes_to(feature_indexes);
    }

    bool has_required_feature() const
    {
        return reqFeatureIndex != 0xFFFFu;
    }
    unsigned int get_required_feature_index() const
    {
        if (reqFeatureIndex == 0xFFFFu)
            return Index::NOT_FOUND_INDEX;
        return reqFeatureIndex;
    }

    bool operator==(const LangSys &o) const
    {
        if (featureIndex.len != o.featureIndex.len || reqFeatureIndex != o.reqFeatureIndex)
            return false;

        for (const auto _ : +hb_zip(featureIndex, o.featureIndex))
            if (_.first != _.second)
                return false;

        return true;
    }

    bool sanitize(hb_sanitize_context_t *c, const Record_sanitize_closure_t * = nullptr) const
    {
        TRACE_SANITIZE(this);
        return_trace(c->check_struct(this) && featureIndex.sanitize(c));
    }

    Offset16 lookupOrderZ;    /* = Null (reserved for an offset to a
                               * reordering table) */
    HBUINT16 reqFeatureIndex; /* Index of a feature required for this
                               * language system--if no required features
                               * = 0xFFFFu */
    IndexArray featureIndex;  /* Array of indices into the FeatureList */
public:
    DEFINE_SIZE_ARRAY_SIZED(6, featureIndex);
};
DECLARE_NULL_NAMESPACE_BYTES(OT, LangSys);

struct Script
{
    unsigned int get_lang_sys_count() const
    {
        return langSys.len;
    }
    const Tag &get_lang_sys_tag(unsigned int i) const
    {
        return langSys.get_tag(i);
    }
    unsigned int get_lang_sys_tags(unsigned int start_offset,
                                   unsigned int *lang_sys_count /* IN/OUT */,
                                   hb_tag_t *lang_sys_tags /* OUT */) const
    {
        return langSys.get_tags(start_offset, lang_sys_count, lang_sys_tags);
    }
    const LangSys &get_lang_sys(unsigned int i) const
    {
        if (i == Index::NOT_FOUND_INDEX)
            return get_default_lang_sys();
        return this + langSys[i].offset;
    }
    bool find_lang_sys_index(hb_tag_t tag, unsigned int *index) const
    {
        return langSys.find_index(tag, index);
    }

    bool has_default_lang_sys() const
    {
        return defaultLangSys != 0;
    }
    const LangSys &get_default_lang_sys() const
    {
        return this + defaultLangSys;
    }

    bool sanitize(hb_sanitize_context_t *c, const Record_sanitize_closure_t * = nullptr) const
    {
        TRACE_SANITIZE(this);
        return_trace(defaultLangSys.sanitize(c, this) && langSys.sanitize(c, this));
    }

protected:
    OffsetTo<LangSys> defaultLangSys; /* Offset to DefaultLangSys table--from
                                       * beginning of Script table--may be Null */
    RecordArrayOf<LangSys> langSys;   /* Array of LangSysRecords--listed
                                       * alphabetically by LangSysTag */
public:
    DEFINE_SIZE_ARRAY_SIZED(4, langSys);
};

typedef RecordListOf<Script> ScriptList;

/* https://docs.microsoft.com/en-us/typography/opentype/spec/features_pt#size */
struct FeatureParamsSize
{
    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        if (unlikely(!c->check_struct(this)))
            return_trace(false);

        /* This subtable has some "history", if you will.  Some earlier versions of
         * Adobe tools calculated the offset of the FeatureParams sutable from the
         * beginning of the FeatureList table!  Now, that is dealt with in the
         * Feature implementation.  But we still need to be able to tell junk from
         * real data.  Note: We don't check that the nameID actually exists.
         *
         * Read Roberts wrote on 9/15/06 on opentype-list@indx.co.uk :
         *
         * Yes, it is correct that a new version of the AFDKO (version 2.0) will be
         * coming out soon, and that the makeotf program will build a font with a
         * 'size' feature that is correct by the specification.
         *
         * The specification for this feature tag is in the "OpenType Layout Tag
         * Registry". You can see a copy of this at:
         * https://docs.microsoft.com/en-us/typography/opentype/spec/features_pt#tag-size
         *
         * Here is one set of rules to determine if the 'size' feature is built
         * correctly, or as by the older versions of MakeOTF. You may be able to do
         * better.
         *
         * Assume that the offset to the size feature is according to specification,
         * and make the following value checks. If it fails, assume the size
         * feature is calculated as versions of MakeOTF before the AFDKO 2.0 built it.
         * If this fails, reject the 'size' feature. The older makeOTF's calculated the
         * offset from the beginning of the FeatureList table, rather than from the
         * beginning of the 'size' Feature table.
         *
         * If "design size" == 0:
         *     fails check
         *
         * Else if ("subfamily identifier" == 0 and
         *     "range start" == 0 and
         *     "range end" == 0 and
         *     "range start" == 0 and
         *     "menu name ID" == 0)
         *     passes check: this is the format used when there is a design size
         * specified, but there is no recommended size range.
         *
         * Else if ("design size" <  "range start" or
         *     "design size" >   "range end" or
         *     "range end" <= "range start" or
         *     "menu name ID"  < 256 or
         *     "menu name ID"  > 32767 or
         *     menu name ID is not a name ID which is actually in the name table)
         *     fails test
         * Else
         *     passes test.
         */

        if (!designSize)
            return_trace(false);
        else if (subfamilyID == 0 && subfamilyNameID == 0 && rangeStart == 0 && rangeEnd == 0)
            return_trace(true);
        else if (designSize < rangeStart || designSize > rangeEnd || subfamilyNameID < 256 || subfamilyNameID > 32767)
            return_trace(false);
        else
            return_trace(true);
    }

    HBUINT16 designSize;    /* Represents the design size in 720/inch
                             * units (decipoints).  The design size entry
                             * must be non-zero.  When there is a design
                             * size but no recommended size range, the
                             * rest of the array will consist of zeros. */
    HBUINT16 subfamilyID;   /* Has no independent meaning, but serves
                             * as an identifier that associates fonts
                             * in a subfamily. All fonts which share a
                             * Preferred or Font Family name and which
                             * differ only by size range shall have the
                             * same subfamily value, and no fonts which
                             * differ in weight or style shall have the
                             * same subfamily value. If this value is
                             * zero, the remaining fields in the array
                             * will be ignored. */
    NameID subfamilyNameID; /* If the preceding value is non-zero, this
                             * value must be set in the range 256 - 32767
                             * (inclusive). It records the value of a
                             * field in the name table, which must
                             * contain English-language strings encoded
                             * in Windows Unicode and Macintosh Roman,
                             * and may contain additional strings
                             * localized to other scripts and languages.
                             * Each of these strings is the name an
                             * application should use, in combination
                             * with the family name, to represent the
                             * subfamily in a menu.  Applications will
                             * choose the appropriate version based on
                             * their selection criteria. */
    HBUINT16 rangeStart;    /* Large end of the recommended usage range
                             * (inclusive), stored in 720/inch units
                             * (decipoints). */
    HBUINT16 rangeEnd;      /* Small end of the recommended usage range
                               (exclusive), stored in 720/inch units
                             * (decipoints). */
public:
    DEFINE_SIZE_STATIC(10);
};

/* https://docs.microsoft.com/en-us/typography/opentype/spec/features_pt#ssxx */
struct FeatureParamsStylisticSet
{
    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        /* Right now minorVersion is at zero.  Which means, any table supports
         * the uiNameID field. */
        return_trace(c->check_struct(this));
    }

    HBUINT16 version; /* (set to 0): This corresponds to a “minor”
                       * version number. Additional data may be
                       * added to the end of this Feature Parameters
                       * table in the future. */

    NameID uiNameID; /* The 'name' table name ID that specifies a
                      * string (or strings, for multiple languages)
                      * for a user-interface label for this
                      * feature.  The values of uiLabelNameId and
                      * sampleTextNameId are expected to be in the
                      * font-specific name ID range (256-32767),
                      * though that is not a requirement in this
                      * Feature Parameters specification. The
                      * user-interface label for the feature can
                      * be provided in multiple languages. An
                      * English string should be included as a
                      * fallback. The string should be kept to a
                      * minimal length to fit comfortably with
                      * different application interfaces. */
public:
    DEFINE_SIZE_STATIC(4);
};

/* https://docs.microsoft.com/en-us/typography/opentype/spec/features_ae#cv01-cv99 */
struct FeatureParamsCharacterVariants
{
    unsigned get_characters(unsigned start_offset, unsigned *char_count, hb_codepoint_t *chars) const
    {
        if (char_count) {
            +characters.sub_array(start_offset, char_count) | hb_sink(hb_array(chars, *char_count));
        }
        return characters.len;
    }

    unsigned get_size() const
    {
        return min_size + characters.len * HBUINT24::static_size;
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        return_trace(c->check_struct(this) && characters.sanitize(c));
    }

    HBUINT16 format;                /* Format number is set to 0. */
    NameID featUILableNameID;       /* The ‘name’ table name ID that
                                     * specifies a string (or strings,
                                     * for multiple languages) for a
                                     * user-interface label for this
                                     * feature. (May be NULL.) */
    NameID featUITooltipTextNameID; /* The ‘name’ table name ID that
                                     * specifies a string (or strings,
                                     * for multiple languages) that an
                                     * application can use for tooltip
                                     * text for this feature. (May be
                                     * nullptr.) */
    NameID sampleTextNameID;        /* The ‘name’ table name ID that
                                     * specifies sample text that
                                     * illustrates the effect of this
                                     * feature. (May be NULL.) */
    HBUINT16 numNamedParameters;    /* Number of named parameters. (May
                                     * be zero.) */
    NameID firstParamUILabelNameID; /* The first ‘name’ table name ID
                                     * used to specify strings for
                                     * user-interface labels for the
                                     * feature parameters. (Must be zero
                                     * if numParameters is zero.) */
    ArrayOf<HBUINT24> characters;   /* Array of the Unicode Scalar Value
                                     * of the characters for which this
                                     * feature provides glyph variants.
                                     * (May be zero.) */
public:
    DEFINE_SIZE_ARRAY(14, characters);
};

struct FeatureParams
{
    bool sanitize(hb_sanitize_context_t *c, hb_tag_t tag) const
    {
#ifdef HB_NO_LAYOUT_FEATURE_PARAMS
        return true;
#endif
        TRACE_SANITIZE(this);
        if (tag == HB_TAG('s', 'i', 'z', 'e'))
            return_trace(u.size.sanitize(c));
        if ((tag & 0xFFFF0000u) == HB_TAG('s', 's', '\0', '\0')) /* ssXX */
            return_trace(u.stylisticSet.sanitize(c));
        if ((tag & 0xFFFF0000u) == HB_TAG('c', 'v', '\0', '\0')) /* cvXX */
            return_trace(u.characterVariants.sanitize(c));
        return_trace(true);
    }

#ifndef HB_NO_LAYOUT_FEATURE_PARAMS
    const FeatureParamsSize &get_size_params(hb_tag_t tag) const
    {
        if (tag == HB_TAG('s', 'i', 'z', 'e'))
            return u.size;
        return Null(FeatureParamsSize);
    }
    const FeatureParamsStylisticSet &get_stylistic_set_params(hb_tag_t tag) const
    {
        if ((tag & 0xFFFF0000u) == HB_TAG('s', 's', '\0', '\0')) /* ssXX */
            return u.stylisticSet;
        return Null(FeatureParamsStylisticSet);
    }
    const FeatureParamsCharacterVariants &get_character_variants_params(hb_tag_t tag) const
    {
        if ((tag & 0xFFFF0000u) == HB_TAG('c', 'v', '\0', '\0')) /* cvXX */
            return u.characterVariants;
        return Null(FeatureParamsCharacterVariants);
    }
#endif

private:
    union {
        FeatureParamsSize size;
        FeatureParamsStylisticSet stylisticSet;
        FeatureParamsCharacterVariants characterVariants;
    } u;

public:
    DEFINE_SIZE_MIN(0);
};

struct Feature
{
    unsigned int get_lookup_count() const
    {
        return lookupIndex.len;
    }
    hb_tag_t get_lookup_index(unsigned int i) const
    {
        return lookupIndex[i];
    }
    unsigned int get_lookup_indexes(unsigned int start_index,
                                    unsigned int *lookup_count /* IN/OUT */,
                                    unsigned int *lookup_tags /* OUT */) const
    {
        return lookupIndex.get_indexes(start_index, lookup_count, lookup_tags);
    }
    void add_lookup_indexes_to(hb_set_t *lookup_indexes) const
    {
        lookupIndex.add_indexes_to(lookup_indexes);
    }

    const FeatureParams &get_feature_params() const
    {
        return this + featureParams;
    }

    bool intersects_lookup_indexes(const hb_map_t *lookup_indexes) const
    {
        return lookupIndex.intersects(lookup_indexes);
    }

    bool sanitize(hb_sanitize_context_t *c, const Record_sanitize_closure_t *closure = nullptr) const
    {
        TRACE_SANITIZE(this);
        if (unlikely(!(c->check_struct(this) && lookupIndex.sanitize(c))))
            return_trace(false);

        /* Some earlier versions of Adobe tools calculated the offset of the
         * FeatureParams subtable from the beginning of the FeatureList table!
         *
         * If sanitizing "failed" for the FeatureParams subtable, try it with the
         * alternative location.  We would know sanitize "failed" if old value
         * of the offset was non-zero, but it's zeroed now.
         *
         * Only do this for the 'size' feature, since at the time of the faulty
         * Adobe tools, only the 'size' feature had FeatureParams defined.
         */

        if (likely(featureParams.is_null()))
            return_trace(true);

        unsigned int orig_offset = featureParams;
        if (unlikely(!featureParams.sanitize(c, this, closure ? closure->tag : HB_TAG_NONE)))
            return_trace(false);

        if (featureParams == 0 && closure && closure->tag == HB_TAG('s', 'i', 'z', 'e') && closure->list_base &&
            closure->list_base < this) {
            unsigned int new_offset_int = orig_offset - (((char *)this) - ((char *)closure->list_base));

            OffsetTo<FeatureParams> new_offset;
            /* Check that it would not overflow. */
            new_offset = new_offset_int;
            if (new_offset == new_offset_int && c->try_set(&featureParams, new_offset_int) &&
                !featureParams.sanitize(c, this, closure ? closure->tag : HB_TAG_NONE))
                return_trace(false);
        }

        return_trace(true);
    }

    OffsetTo<FeatureParams> featureParams; /* Offset to Feature Parameters table (if one
                                            * has been defined for the feature), relative
                                            * to the beginning of the Feature Table; = Null
                                            * if not required */
    IndexArray lookupIndex;                /* Array of LookupList indices */
public:
    DEFINE_SIZE_ARRAY_SIZED(4, lookupIndex);
};

typedef RecordListOf<Feature> FeatureList;

struct LookupFlag : HBUINT16
{
    enum Flags {
        RightToLeft = 0x0001u,
        IgnoreBaseGlyphs = 0x0002u,
        IgnoreLigatures = 0x0004u,
        IgnoreMarks = 0x0008u,
        IgnoreFlags = 0x000Eu,
        UseMarkFilteringSet = 0x0010u,
        Reserved = 0x00E0u,
        MarkAttachmentType = 0xFF00u
    };

public:
    DEFINE_SIZE_STATIC(2);
};

} /* namespace OT */
/* This has to be outside the namespace. */
HB_MARK_AS_FLAG_T(OT::LookupFlag::Flags);
namespace OT {

struct Lookup
{
    unsigned int get_subtable_count() const
    {
        return subTable.len;
    }

    template <typename TSubTable> const OffsetArrayOf<TSubTable> &get_subtables() const
    {
        return reinterpret_cast<const OffsetArrayOf<TSubTable> &>(subTable);
    }
    template <typename TSubTable> OffsetArrayOf<TSubTable> &get_subtables()
    {
        return reinterpret_cast<OffsetArrayOf<TSubTable> &>(subTable);
    }

    template <typename TSubTable> const TSubTable &get_subtable(unsigned int i) const
    {
        return this + get_subtables<TSubTable>()[i];
    }
    template <typename TSubTable> TSubTable &get_subtable(unsigned int i)
    {
        return this + get_subtables<TSubTable>()[i];
    }

    unsigned int get_size() const
    {
        const HBUINT16 &markFilteringSet = StructAfter<const HBUINT16>(subTable);
        if (lookupFlag & LookupFlag::UseMarkFilteringSet)
            return (const char *)&StructAfter<const char>(markFilteringSet) - (const char *)this;
        return (const char *)&markFilteringSet - (const char *)this;
    }

    unsigned int get_type() const
    {
        return lookupType;
    }

    /* lookup_props is a 32-bit integer where the lower 16-bit is LookupFlag and
     * higher 16-bit is mark-filtering-set if the lookup uses one.
     * Not to be confused with glyph_props which is very similar. */
    uint32_t get_props() const
    {
        unsigned int flag = lookupFlag;
        if (unlikely(flag & LookupFlag::UseMarkFilteringSet)) {
            const HBUINT16 &markFilteringSet = StructAfter<HBUINT16>(subTable);
            flag += (markFilteringSet << 16);
        }
        return flag;
    }

    template <typename TSubTable, typename context_t, typename... Ts>
    typename context_t::return_t dispatch(context_t *c, Ts &&... ds) const
    {
        unsigned int lookup_type = get_type();
        TRACE_DISPATCH(this, lookup_type);
        unsigned int count = get_subtable_count();
        for (unsigned int i = 0; i < count; i++) {
            typename context_t::return_t r = get_subtable<TSubTable>(i).dispatch(c, lookup_type, hb_forward<Ts>(ds)...);
            if (c->stop_sublookup_iteration(r))
                return_trace(r);
        }
        return_trace(c->default_return_value());
    }

    template <typename TSubTable> bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        if (!(c->check_struct(this) && subTable.sanitize(c)))
            return_trace(false);

        unsigned subtables = get_subtable_count();
        if (unlikely(!c->visit_subtables(subtables)))
            return_trace(false);

        if (lookupFlag & LookupFlag::UseMarkFilteringSet) {
            const HBUINT16 &markFilteringSet = StructAfter<HBUINT16>(subTable);
            if (!markFilteringSet.sanitize(c))
                return_trace(false);
        }

        if (unlikely(!get_subtables<TSubTable>().sanitize(c, this, get_type())))
            return_trace(false);

        if (unlikely(get_type() == TSubTable::Extension && !c->get_edit_count())) {
            /* The spec says all subtables of an Extension lookup should
             * have the same type, which shall not be the Extension type
             * itself (but we already checked for that).
             * This is specially important if one has a reverse type!
             *
             * We only do this if sanitizer edit_count is zero.  Otherwise,
             * some of the subtables might have become insane after they
             * were sanity-checked by the edits of subsequent subtables.
             * https://bugs.chromium.org/p/chromium/issues/detail?id=960331
             */
            unsigned int type = get_subtable<TSubTable>(0).u.extension.get_type();
            for (unsigned int i = 1; i < subtables; i++)
                if (get_subtable<TSubTable>(i).u.extension.get_type() != type)
                    return_trace(false);
        }
        return_trace(true);
    }

private:
    HBUINT16 lookupType;                           /* Different enumerations for GSUB and GPOS */
    HBUINT16 lookupFlag;                           /* Lookup qualifiers */
    ArrayOf<Offset16> subTable;                    /* Array of SubTables */
    /*HBUINT16	markFilteringSetX[HB_VAR_ARRAY];*/ /* Index (base 0) into GDEF mark glyph sets
                                                    * structure. This field is only present if bit
                                                    * UseMarkFilteringSet of lookup flags is set. */
public:
    DEFINE_SIZE_ARRAY(6, subTable);
};

typedef OffsetListOf<Lookup> LookupList;

template <typename TLookup> struct LookupOffsetList : OffsetListOf<TLookup>
{
    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        return_trace(OffsetListOf<TLookup>::sanitize(c, this));
    }
};

/*
 * Coverage Table
 */

struct CoverageFormat1
{
    friend struct Coverage;

private:
    unsigned int get_coverage(hb_codepoint_t glyph_id) const
    {
        unsigned int i;
        glyphArray.bfind(glyph_id, &i, HB_BFIND_NOT_FOUND_STORE, NOT_COVERED);
        return i;
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        return_trace(glyphArray.sanitize(c));
    }

    bool intersects(const hb_set_t *glyphs) const
    {
        /* TODO Speed up, using hb_set_next() and bsearch()? */
        unsigned int count = glyphArray.len;
        for (unsigned int i = 0; i < count; i++)
            if (glyphs->has(glyphArray[i]))
                return true;
        return false;
    }
    bool intersects_coverage(const hb_set_t *glyphs, unsigned int index) const
    {
        return glyphs->has(glyphArray[index]);
    }

    template <typename set_t> bool collect_coverage(set_t *glyphs) const
    {
        return glyphs->add_sorted_array(glyphArray.arrayZ, glyphArray.len);
    }

public:
    /* Older compilers need this to be public. */
    struct iter_t
    {
        void init(const struct CoverageFormat1 &c_)
        {
            c = &c_;
            i = 0;
        }
        void fini() {}
        bool more() const
        {
            return i < c->glyphArray.len;
        }
        void next()
        {
            i++;
        }
        hb_codepoint_t get_glyph() const
        {
            return c->glyphArray[i];
        }
        bool operator!=(const iter_t &o) const
        {
            return i != o.i || c != o.c;
        }

    private:
        const struct CoverageFormat1 *c;
        unsigned int i;
    };

private:
protected:
    HBUINT16 coverageFormat;             /* Format identifier--format = 1 */
    SortedArrayOf<HBGlyphID> glyphArray; /* Array of GlyphIDs--in numerical order */
public:
    DEFINE_SIZE_ARRAY(4, glyphArray);
};

struct CoverageFormat2
{
    friend struct Coverage;

private:
    unsigned int get_coverage(hb_codepoint_t glyph_id) const
    {
        const RangeRecord &range = rangeRecord.bsearch(glyph_id);
        return likely(range.first <= range.last) ? (unsigned int)range.value + (glyph_id - range.first) : NOT_COVERED;
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        return_trace(rangeRecord.sanitize(c));
    }

    bool intersects(const hb_set_t *glyphs) const
    {
        /* TODO Speed up, using hb_set_next() and bsearch()? */
        unsigned int count = rangeRecord.len;
        for (unsigned int i = 0; i < count; i++)
            if (rangeRecord[i].intersects(glyphs))
                return true;
        return false;
    }
    bool intersects_coverage(const hb_set_t *glyphs, unsigned int index) const
    {
        unsigned int i;
        unsigned int count = rangeRecord.len;
        for (i = 0; i < count; i++) {
            const RangeRecord &range = rangeRecord[i];
            if (range.value <= index && index < (unsigned int)range.value + (range.last - range.first) &&
                range.intersects(glyphs))
                return true;
            else if (index < range.value)
                return false;
        }
        return false;
    }

    template <typename set_t> bool collect_coverage(set_t *glyphs) const
    {
        unsigned int count = rangeRecord.len;
        for (unsigned int i = 0; i < count; i++)
            if (unlikely(!rangeRecord[i].collect_coverage(glyphs)))
                return false;
        return true;
    }

public:
    /* Older compilers need this to be public. */
    struct iter_t
    {
        void init(const CoverageFormat2 &c_)
        {
            c = &c_;
            coverage = 0;
            i = 0;
            j = c->rangeRecord.len ? c->rangeRecord[0].first : 0;
            if (unlikely(c->rangeRecord[0].first > c->rangeRecord[0].last)) {
                /* Broken table. Skip. */
                i = c->rangeRecord.len;
            }
        }
        void fini() {}
        bool more() const
        {
            return i < c->rangeRecord.len;
        }
        void next()
        {
            if (j >= c->rangeRecord[i].last) {
                i++;
                if (more()) {
                    unsigned int old = coverage;
                    j = c->rangeRecord[i].first;
                    coverage = c->rangeRecord[i].value;
                    if (unlikely(coverage != old + 1)) {
                        /* Broken table. Skip. Important to avoid DoS.
                         * Also, our callers depend on coverage being
                         * consecutive and monotonically increasing,
                         * ie. iota(). */
                        i = c->rangeRecord.len;
                        return;
                    }
                }
                return;
            }
            coverage++;
            j++;
        }
        hb_codepoint_t get_glyph() const
        {
            return j;
        }
        bool operator!=(const iter_t &o) const
        {
            return i != o.i || j != o.j || c != o.c;
        }

    private:
        const struct CoverageFormat2 *c;
        unsigned int i, coverage;
        hb_codepoint_t j;
    };

private:
protected:
    HBUINT16 coverageFormat;                /* Format identifier--format = 2 */
    SortedArrayOf<RangeRecord> rangeRecord; /* Array of glyph ranges--ordered by
                                             * Start GlyphID. rangeCount entries
                                             * long */
public:
    DEFINE_SIZE_ARRAY(4, rangeRecord);
};

struct Coverage
{
    /* Has interface. */
    static constexpr unsigned SENTINEL = NOT_COVERED;
    typedef unsigned int value_t;
    value_t operator[](hb_codepoint_t k) const
    {
        return get(k);
    }
    bool has(hb_codepoint_t k) const
    {
        return (*this)[k] != SENTINEL;
    }
    /* Predicate. */
    bool operator()(hb_codepoint_t k) const
    {
        return has(k);
    }

    unsigned int get(hb_codepoint_t k) const
    {
        return get_coverage(k);
    }
    unsigned int get_coverage(hb_codepoint_t glyph_id) const
    {
        switch (u.format) {
        case 1:
            return u.format1.get_coverage(glyph_id);
        case 2:
            return u.format2.get_coverage(glyph_id);
        default:
            return NOT_COVERED;
        }
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        if (!u.format.sanitize(c))
            return_trace(false);
        switch (u.format) {
        case 1:
            return_trace(u.format1.sanitize(c));
        case 2:
            return_trace(u.format2.sanitize(c));
        default:
            return_trace(true);
        }
    }

    bool intersects(const hb_set_t *glyphs) const
    {
        switch (u.format) {
        case 1:
            return u.format1.intersects(glyphs);
        case 2:
            return u.format2.intersects(glyphs);
        default:
            return false;
        }
    }
    bool intersects_coverage(const hb_set_t *glyphs, unsigned int index) const
    {
        switch (u.format) {
        case 1:
            return u.format1.intersects_coverage(glyphs, index);
        case 2:
            return u.format2.intersects_coverage(glyphs, index);
        default:
            return false;
        }
    }

    /* Might return false if array looks unsorted.
     * Used for faster rejection of corrupt data. */
    template <typename set_t> bool collect_coverage(set_t *glyphs) const
    {
        switch (u.format) {
        case 1:
            return u.format1.collect_coverage(glyphs);
        case 2:
            return u.format2.collect_coverage(glyphs);
        default:
            return false;
        }
    }

    struct iter_t : hb_iter_with_fallback_t<iter_t, hb_codepoint_t>
    {
        static constexpr bool is_sorted_iterator = true;
        iter_t(const Coverage &c_ = Null(Coverage))
        {
            memset(this, 0, sizeof(*this));
            format = c_.u.format;
            switch (format) {
            case 1:
                u.format1.init(c_.u.format1);
                return;
            case 2:
                u.format2.init(c_.u.format2);
                return;
            default:
                return;
            }
        }
        bool __more__() const
        {
            switch (format) {
            case 1:
                return u.format1.more();
            case 2:
                return u.format2.more();
            default:
                return false;
            }
        }
        void __next__()
        {
            switch (format) {
            case 1:
                u.format1.next();
                break;
            case 2:
                u.format2.next();
                break;
            default:
                break;
            }
        }
        typedef hb_codepoint_t __item_t__;
        __item_t__ __item__() const
        {
            return get_glyph();
        }

        hb_codepoint_t get_glyph() const
        {
            switch (format) {
            case 1:
                return u.format1.get_glyph();
            case 2:
                return u.format2.get_glyph();
            default:
                return 0;
            }
        }
        bool operator!=(const iter_t &o) const
        {
            if (format != o.format)
                return true;
            switch (format) {
            case 1:
                return u.format1 != o.u.format1;
            case 2:
                return u.format2 != o.u.format2;
            default:
                return false;
            }
        }

    private:
        unsigned int format;
        union {
            CoverageFormat2::iter_t format2; /* Put this one first since it's larger; helps shut up compiler. */
            CoverageFormat1::iter_t format1;
        } u;
    };
    iter_t iter() const
    {
        return iter_t(*this);
    }

protected:
    union {
        HBUINT16 format; /* Format identifier */
        CoverageFormat1 format1;
        CoverageFormat2 format2;
    } u;

public:
    DEFINE_SIZE_UNION(2, format);
};

/*
 * Class Definition Table
 */

struct ClassDefFormat1
{
    friend struct ClassDef;

private:
    unsigned int get_class(hb_codepoint_t glyph_id) const
    {
        return classValue[(unsigned int)(glyph_id - startGlyph)];
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        return_trace(c->check_struct(this) && classValue.sanitize(c));
    }

    template <typename set_t> bool collect_coverage(set_t *glyphs) const
    {
        unsigned int start = 0;
        unsigned int count = classValue.len;
        for (unsigned int i = 0; i < count; i++) {
            if (classValue[i])
                continue;

            if (start != i)
                if (unlikely(!glyphs->add_range(startGlyph + start, startGlyph + i)))
                    return false;

            start = i + 1;
        }
        if (start != count)
            if (unlikely(!glyphs->add_range(startGlyph + start, startGlyph + count)))
                return false;

        return true;
    }

    template <typename set_t> bool collect_class(set_t *glyphs, unsigned int klass) const
    {
        unsigned int count = classValue.len;
        for (unsigned int i = 0; i < count; i++)
            if (classValue[i] == klass)
                glyphs->add(startGlyph + i);
        return true;
    }

    bool intersects(const hb_set_t *glyphs) const
    {
        /* TODO Speed up, using hb_set_next()? */
        hb_codepoint_t start = startGlyph;
        hb_codepoint_t end = startGlyph + classValue.len;
        for (hb_codepoint_t iter = startGlyph - 1; hb_set_next(glyphs, &iter) && iter < end;)
            if (classValue[iter - start])
                return true;
        return false;
    }
    bool intersects_class(const hb_set_t *glyphs, unsigned int klass) const
    {
        unsigned int count = classValue.len;
        if (klass == 0) {
            /* Match if there's any glyph that is not listed! */
            hb_codepoint_t g = HB_SET_VALUE_INVALID;
            if (!hb_set_next(glyphs, &g))
                return false;
            if (g < startGlyph)
                return true;
            g = startGlyph + count - 1;
            if (hb_set_next(glyphs, &g))
                return true;
            /* Fall through. */
        }
        for (unsigned int i = 0; i < count; i++)
            if (classValue[i] == klass && glyphs->has(startGlyph + i))
                return true;
        return false;
    }

protected:
    HBUINT16 classFormat;         /* Format identifier--format = 1 */
    HBGlyphID startGlyph;         /* First GlyphID of the classValueArray */
    ArrayOf<HBUINT16> classValue; /* Array of Class Values--one per GlyphID */
public:
    DEFINE_SIZE_ARRAY(6, classValue);
};

struct ClassDefFormat2
{
    friend struct ClassDef;

private:
    unsigned int get_class(hb_codepoint_t glyph_id) const
    {
        return rangeRecord.bsearch(glyph_id).value;
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        return_trace(rangeRecord.sanitize(c));
    }

    template <typename set_t> bool collect_coverage(set_t *glyphs) const
    {
        unsigned int count = rangeRecord.len;
        for (unsigned int i = 0; i < count; i++)
            if (rangeRecord[i].value)
                if (unlikely(!rangeRecord[i].collect_coverage(glyphs)))
                    return false;
        return true;
    }

    template <typename set_t> bool collect_class(set_t *glyphs, unsigned int klass) const
    {
        unsigned int count = rangeRecord.len;
        for (unsigned int i = 0; i < count; i++) {
            if (rangeRecord[i].value == klass)
                if (unlikely(!rangeRecord[i].collect_coverage(glyphs)))
                    return false;
        }
        return true;
    }

    bool intersects(const hb_set_t *glyphs) const
    {
        /* TODO Speed up, using hb_set_next() and bsearch()? */
        unsigned int count = rangeRecord.len;
        for (unsigned int i = 0; i < count; i++)
            if (rangeRecord[i].intersects(glyphs))
                return true;
        return false;
    }
    bool intersects_class(const hb_set_t *glyphs, unsigned int klass) const
    {
        unsigned int count = rangeRecord.len;
        if (klass == 0) {
            /* Match if there's any glyph that is not listed! */
            hb_codepoint_t g = HB_SET_VALUE_INVALID;
            for (unsigned int i = 0; i < count; i++) {
                if (!hb_set_next(glyphs, &g))
                    break;
                if (g < rangeRecord[i].first)
                    return true;
                g = rangeRecord[i].last;
            }
            if (g != HB_SET_VALUE_INVALID && hb_set_next(glyphs, &g))
                return true;
            /* Fall through. */
        }
        for (unsigned int i = 0; i < count; i++)
            if (rangeRecord[i].value == klass && rangeRecord[i].intersects(glyphs))
                return true;
        return false;
    }

protected:
    HBUINT16 classFormat;                   /* Format identifier--format = 2 */
    SortedArrayOf<RangeRecord> rangeRecord; /* Array of glyph ranges--ordered by
                                             * Start GlyphID */
public:
    DEFINE_SIZE_ARRAY(4, rangeRecord);
};

struct ClassDef
{
    /* Has interface. */
    static constexpr unsigned SENTINEL = 0;
    typedef unsigned int value_t;
    value_t operator[](hb_codepoint_t k) const
    {
        return get(k);
    }
    bool has(hb_codepoint_t k) const
    {
        return (*this)[k] != SENTINEL;
    }
    /* Projection. */
    hb_codepoint_t operator()(hb_codepoint_t k) const
    {
        return get(k);
    }

    unsigned int get(hb_codepoint_t k) const
    {
        return get_class(k);
    }
    unsigned int get_class(hb_codepoint_t glyph_id) const
    {
        switch (u.format) {
        case 1:
            return u.format1.get_class(glyph_id);
        case 2:
            return u.format2.get_class(glyph_id);
        default:
            return 0;
        }
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        if (!u.format.sanitize(c))
            return_trace(false);
        switch (u.format) {
        case 1:
            return_trace(u.format1.sanitize(c));
        case 2:
            return_trace(u.format2.sanitize(c));
        default:
            return_trace(true);
        }
    }

    /* Might return false if array looks unsorted.
     * Used for faster rejection of corrupt data. */
    template <typename set_t> bool collect_coverage(set_t *glyphs) const
    {
        switch (u.format) {
        case 1:
            return u.format1.collect_coverage(glyphs);
        case 2:
            return u.format2.collect_coverage(glyphs);
        default:
            return false;
        }
    }

    /* Might return false if array looks unsorted.
     * Used for faster rejection of corrupt data. */
    template <typename set_t> bool collect_class(set_t *glyphs, unsigned int klass) const
    {
        switch (u.format) {
        case 1:
            return u.format1.collect_class(glyphs, klass);
        case 2:
            return u.format2.collect_class(glyphs, klass);
        default:
            return false;
        }
    }

    bool intersects(const hb_set_t *glyphs) const
    {
        switch (u.format) {
        case 1:
            return u.format1.intersects(glyphs);
        case 2:
            return u.format2.intersects(glyphs);
        default:
            return false;
        }
    }
    bool intersects_class(const hb_set_t *glyphs, unsigned int klass) const
    {
        switch (u.format) {
        case 1:
            return u.format1.intersects_class(glyphs, klass);
        case 2:
            return u.format2.intersects_class(glyphs, klass);
        default:
            return false;
        }
    }

protected:
    union {
        HBUINT16 format; /* Format identifier */
        ClassDefFormat1 format1;
        ClassDefFormat2 format2;
    } u;

public:
    DEFINE_SIZE_UNION(2, format);
};

/*
 * Item Variation Store
 */

struct VarRegionAxis
{
    float evaluate(int coord) const
    {
        int start = startCoord, peak = peakCoord, end = endCoord;

        /* TODO Move these to sanitize(). */
        if (unlikely(start > peak || peak > end))
            return 1.;
        if (unlikely(start < 0 && end > 0 && peak != 0))
            return 1.;

        if (peak == 0 || coord == peak)
            return 1.;

        if (coord <= start || end <= coord)
            return 0.;

        /* Interpolate */
        if (coord < peak)
            return float(coord - start) / (peak - start);
        else
            return float(end - coord) / (end - peak);
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        return_trace(c->check_struct(this));
        /* TODO Handle invalid start/peak/end configs, so we don't
         * have to do that at runtime. */
    }

public:
    F2DOT14 startCoord;
    F2DOT14 peakCoord;
    F2DOT14 endCoord;

public:
    DEFINE_SIZE_STATIC(6);
};

struct VarRegionList
{
    float evaluate(unsigned int region_index, const int *coords, unsigned int coord_len) const
    {
        if (unlikely(region_index >= regionCount))
            return 0.;

        const VarRegionAxis *axes = axesZ.arrayZ + (region_index * axisCount);

        float v = 1.;
        unsigned int count = axisCount;
        for (unsigned int i = 0; i < count; i++) {
            int coord = i < coord_len ? coords[i] : 0;
            float factor = axes[i].evaluate(coord);
            if (factor == 0.f)
                return 0.;
            v *= factor;
        }
        return v;
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        return_trace(c->check_struct(this) && axesZ.sanitize(c, (unsigned int)axisCount * (unsigned int)regionCount));
    }

    unsigned int get_size() const
    {
        return min_size + VarRegionAxis::static_size * axisCount * regionCount;
    }
    unsigned int get_region_count() const
    {
        return regionCount;
    }

protected:
    HBUINT16 axisCount;
    HBUINT16 regionCount;
    UnsizedArrayOf<VarRegionAxis> axesZ;

public:
    DEFINE_SIZE_ARRAY(4, axesZ);
};

struct VarData
{
    unsigned int get_region_index_count() const
    {
        return regionIndices.len;
    }

    unsigned int get_row_size() const
    {
        return shortCount + regionIndices.len;
    }

    unsigned int get_size() const
    {
        return itemCount * get_row_size();
    }

    float get_delta(unsigned int inner, const int *coords, unsigned int coord_count, const VarRegionList &regions) const
    {
        if (unlikely(inner >= itemCount))
            return 0.;

        unsigned int count = regionIndices.len;
        unsigned int scount = shortCount;

        const HBUINT8 *bytes = get_delta_bytes();
        const HBUINT8 *row = bytes + inner * (scount + count);

        float delta = 0.;
        unsigned int i = 0;

        const HBINT16 *scursor = reinterpret_cast<const HBINT16 *>(row);
        for (; i < scount; i++) {
            float scalar = regions.evaluate(regionIndices.arrayZ[i], coords, coord_count);
            delta += scalar * *scursor++;
        }
        const HBINT8 *bcursor = reinterpret_cast<const HBINT8 *>(scursor);
        for (; i < count; i++) {
            float scalar = regions.evaluate(regionIndices.arrayZ[i], coords, coord_count);
            delta += scalar * *bcursor++;
        }

        return delta;
    }

    void get_scalars(const int *coords,
                     unsigned int coord_count,
                     const VarRegionList &regions,
                     float *scalars /*OUT */,
                     unsigned int num_scalars) const
    {
        unsigned count = hb_min(num_scalars, regionIndices.len);
        for (unsigned int i = 0; i < count; i++)
            scalars[i] = regions.evaluate(regionIndices.arrayZ[i], coords, coord_count);
        for (unsigned int i = count; i < num_scalars; i++)
            scalars[i] = 0.f;
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        return_trace(c->check_struct(this) && regionIndices.sanitize(c) && shortCount <= regionIndices.len &&
                     c->check_range(get_delta_bytes(), itemCount, get_row_size()));
    }

protected:
    const HBUINT8 *get_delta_bytes() const
    {
        return &StructAfter<HBUINT8>(regionIndices);
    }

    HBUINT8 *get_delta_bytes()
    {
        return &StructAfter<HBUINT8>(regionIndices);
    }

    int16_t get_item_delta(unsigned int item, unsigned int region) const
    {
        if (item >= itemCount || unlikely(region >= regionIndices.len))
            return 0;
        const HBINT8 *p = (const HBINT8 *)get_delta_bytes() + item * get_row_size();
        if (region < shortCount)
            return ((const HBINT16 *)p)[region];
        else
            return (p + HBINT16::static_size * shortCount)[region - shortCount];
    }

    void set_item_delta(unsigned int item, unsigned int region, int16_t delta)
    {
        HBINT8 *p = (HBINT8 *)get_delta_bytes() + item * get_row_size();
        if (region < shortCount)
            ((HBINT16 *)p)[region] = delta;
        else
            (p + HBINT16::static_size * shortCount)[region - shortCount] = delta;
    }

protected:
    HBUINT16 itemCount;
    HBUINT16 shortCount;
    ArrayOf<HBUINT16> regionIndices;
    /*UnsizedArrayOf<HBUINT8>bytesX;*/
public:
    DEFINE_SIZE_ARRAY(6, regionIndices);
};

struct VariationStore
{
    float get_delta(unsigned int outer, unsigned int inner, const int *coords, unsigned int coord_count) const
    {
        if (unlikely(outer >= dataSets.len))
            return 0.f;

        return (this + dataSets[outer]).get_delta(inner, coords, coord_count, this + regions);
    }

    float get_delta(unsigned int index, const int *coords, unsigned int coord_count) const
    {
        unsigned int outer = index >> 16;
        unsigned int inner = index & 0xFFFF;
        return get_delta(outer, inner, coords, coord_count);
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        return_trace(c->check_struct(this) && format == 1 && regions.sanitize(c, this) && dataSets.sanitize(c, this));
    }

    unsigned int get_region_index_count(unsigned int ivs) const
    {
        return (this + dataSets[ivs]).get_region_index_count();
    }

    void get_scalars(unsigned int ivs,
                     const int *coords,
                     unsigned int coord_count,
                     float *scalars /*OUT*/,
                     unsigned int num_scalars) const
    {
        (this + dataSets[ivs]).get_scalars(coords, coord_count, this + regions, &scalars[0], num_scalars);
    }

    unsigned int get_sub_table_count() const
    {
        return dataSets.len;
    }

protected:
    HBUINT16 format;
    LOffsetTo<VarRegionList> regions;
    LOffsetArrayOf<VarData> dataSets;

public:
    DEFINE_SIZE_ARRAY(8, dataSets);
};

/*
 * Feature Variations
 */

struct ConditionFormat1
{
    friend struct Condition;

private:
    bool evaluate(const int *coords, unsigned int coord_len) const
    {
        int coord = axisIndex < coord_len ? coords[axisIndex] : 0;
        return filterRangeMinValue <= coord && coord <= filterRangeMaxValue;
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        return_trace(c->check_struct(this));
    }

protected:
    HBUINT16 format; /* Format identifier--format = 1 */
    HBUINT16 axisIndex;
    F2DOT14 filterRangeMinValue;
    F2DOT14 filterRangeMaxValue;

public:
    DEFINE_SIZE_STATIC(8);
};

struct Condition
{
    bool evaluate(const int *coords, unsigned int coord_len) const
    {
        switch (u.format) {
        case 1:
            return u.format1.evaluate(coords, coord_len);
        default:
            return false;
        }
    }

    template <typename context_t, typename... Ts> typename context_t::return_t dispatch(context_t *c, Ts &&... ds) const
    {
        TRACE_DISPATCH(this, u.format);
        if (unlikely(!c->may_dispatch(this, &u.format)))
            return_trace(c->no_dispatch_return_value());
        switch (u.format) {
        case 1:
            return_trace(c->dispatch(u.format1, hb_forward<Ts>(ds)...));
        default:
            return_trace(c->default_return_value());
        }
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        if (!u.format.sanitize(c))
            return_trace(false);
        switch (u.format) {
        case 1:
            return_trace(u.format1.sanitize(c));
        default:
            return_trace(true);
        }
    }

protected:
    union {
        HBUINT16 format; /* Format identifier */
        ConditionFormat1 format1;
    } u;

public:
    DEFINE_SIZE_UNION(2, format);
};

struct ConditionSet
{
    bool evaluate(const int *coords, unsigned int coord_len) const
    {
        unsigned int count = conditions.len;
        for (unsigned int i = 0; i < count; i++)
            if (!(this + conditions.arrayZ[i]).evaluate(coords, coord_len))
                return false;
        return true;
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        return_trace(conditions.sanitize(c, this));
    }

protected:
    LOffsetArrayOf<Condition> conditions;

public:
    DEFINE_SIZE_ARRAY(2, conditions);
};

struct FeatureTableSubstitutionRecord
{
    friend struct FeatureTableSubstitution;

    void collect_lookups(const void *base, hb_set_t *lookup_indexes /* OUT */) const
    {
        return (base + feature).add_lookup_indexes_to(lookup_indexes);
    }

    void closure_features(const void *base, const hb_map_t *lookup_indexes, hb_set_t *feature_indexes /* OUT */) const
    {
        if ((base + feature).intersects_lookup_indexes(lookup_indexes))
            feature_indexes->add(featureIndex);
    }

    bool sanitize(hb_sanitize_context_t *c, const void *base) const
    {
        TRACE_SANITIZE(this);
        return_trace(c->check_struct(this) && feature.sanitize(c, base));
    }

protected:
    HBUINT16 featureIndex;
    LOffsetTo<Feature> feature;

public:
    DEFINE_SIZE_STATIC(6);
};

struct FeatureTableSubstitution
{
    const Feature *find_substitute(unsigned int feature_index) const
    {
        unsigned int count = substitutions.len;
        for (unsigned int i = 0; i < count; i++) {
            const FeatureTableSubstitutionRecord &record = substitutions.arrayZ[i];
            if (record.featureIndex == feature_index)
                return &(this + record.feature);
        }
        return nullptr;
    }

    void collect_lookups(const hb_set_t *feature_indexes, hb_set_t *lookup_indexes /* OUT */) const
    {
        +hb_iter(substitutions) | hb_filter(feature_indexes, &FeatureTableSubstitutionRecord::featureIndex) |
            hb_apply([this, lookup_indexes](const FeatureTableSubstitutionRecord &r) {
                r.collect_lookups(this, lookup_indexes);
            });
    }

    void closure_features(const hb_map_t *lookup_indexes, hb_set_t *feature_indexes /* OUT */) const
    {
        for (const FeatureTableSubstitutionRecord &record : substitutions)
            record.closure_features(this, lookup_indexes, feature_indexes);
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        return_trace(version.sanitize(c) && likely(version.major == 1) && substitutions.sanitize(c, this));
    }

protected:
    FixedVersion<> version; /* Version--0x00010000u */
    ArrayOf<FeatureTableSubstitutionRecord> substitutions;

public:
    DEFINE_SIZE_ARRAY(6, substitutions);
};

struct FeatureVariationRecord
{
    friend struct FeatureVariations;

    void collect_lookups(const void *base, const hb_set_t *feature_indexes, hb_set_t *lookup_indexes /* OUT */) const
    {
        return (base + substitutions).collect_lookups(feature_indexes, lookup_indexes);
    }

    void closure_features(const void *base, const hb_map_t *lookup_indexes, hb_set_t *feature_indexes /* OUT */) const
    {
        (base + substitutions).closure_features(lookup_indexes, feature_indexes);
    }

    bool sanitize(hb_sanitize_context_t *c, const void *base) const
    {
        TRACE_SANITIZE(this);
        return_trace(conditions.sanitize(c, base) && substitutions.sanitize(c, base));
    }

protected:
    LOffsetTo<ConditionSet> conditions;
    LOffsetTo<FeatureTableSubstitution> substitutions;

public:
    DEFINE_SIZE_STATIC(8);
};

struct FeatureVariations
{
    static constexpr unsigned NOT_FOUND_INDEX = 0xFFFFFFFFu;

    bool find_index(const int *coords, unsigned int coord_len, unsigned int *index) const
    {
        unsigned int count = varRecords.len;
        for (unsigned int i = 0; i < count; i++) {
            const FeatureVariationRecord &record = varRecords.arrayZ[i];
            if ((this + record.conditions).evaluate(coords, coord_len)) {
                *index = i;
                return true;
            }
        }
        *index = NOT_FOUND_INDEX;
        return false;
    }

    const Feature *find_substitute(unsigned int variations_index, unsigned int feature_index) const
    {
        const FeatureVariationRecord &record = varRecords[variations_index];
        return (this + record.substitutions).find_substitute(feature_index);
    }

    void collect_lookups(const hb_set_t *feature_indexes, hb_set_t *lookup_indexes /* OUT */) const
    {
        for (const FeatureVariationRecord &r : varRecords)
            r.collect_lookups(this, feature_indexes, lookup_indexes);
    }

    void closure_features(const hb_map_t *lookup_indexes, hb_set_t *feature_indexes /* OUT */) const
    {
        for (const FeatureVariationRecord &record : varRecords)
            record.closure_features(this, lookup_indexes, feature_indexes);
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        return_trace(version.sanitize(c) && likely(version.major == 1) && varRecords.sanitize(c, this));
    }

protected:
    FixedVersion<> version; /* Version--0x00010000u */
    LArrayOf<FeatureVariationRecord> varRecords;

public:
    DEFINE_SIZE_ARRAY_SIZED(8, varRecords);
};

/*
 * Device Tables
 */

struct HintingDevice
{
    friend struct Device;

private:
    hb_position_t get_x_delta(hb_font_t *font) const
    {
        return get_delta(font->x_ppem, font->x_scale);
    }

    hb_position_t get_y_delta(hb_font_t *font) const
    {
        return get_delta(font->y_ppem, font->y_scale);
    }

public:
    unsigned int get_size() const
    {
        unsigned int f = deltaFormat;
        if (unlikely(f < 1 || f > 3 || startSize > endSize))
            return 3 * HBUINT16::static_size;
        return HBUINT16::static_size * (4 + ((endSize - startSize) >> (4 - f)));
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        return_trace(c->check_struct(this) && c->check_range(this, this->get_size()));
    }

private:
    int get_delta(unsigned int ppem, int scale) const
    {
        if (!ppem)
            return 0;

        int pixels = get_delta_pixels(ppem);

        if (!pixels)
            return 0;

        return (int)(pixels * (int64_t)scale / ppem);
    }
    int get_delta_pixels(unsigned int ppem_size) const
    {
        unsigned int f = deltaFormat;
        if (unlikely(f < 1 || f > 3))
            return 0;

        if (ppem_size < startSize || ppem_size > endSize)
            return 0;

        unsigned int s = ppem_size - startSize;

        unsigned int byte = deltaValueZ[s >> (4 - f)];
        unsigned int bits = (byte >> (16 - (((s & ((1 << (4 - f)) - 1)) + 1) << f)));
        unsigned int mask = (0xFFFFu >> (16 - (1 << f)));

        int delta = bits & mask;

        if ((unsigned int)delta >= ((mask + 1) >> 1))
            delta -= mask + 1;

        return delta;
    }

protected:
    HBUINT16 startSize;                   /* Smallest size to correct--in ppem */
    HBUINT16 endSize;                     /* Largest size to correct--in ppem */
    HBUINT16 deltaFormat;                 /* Format of DeltaValue array data: 1, 2, or 3
                                           * 1	Signed 2-bit value, 8 values per uint16
                                           * 2	Signed 4-bit value, 4 values per uint16
                                           * 3	Signed 8-bit value, 2 values per uint16
                                           */
    UnsizedArrayOf<HBUINT16> deltaValueZ; /* Array of compressed data */
public:
    DEFINE_SIZE_ARRAY(6, deltaValueZ);
};

struct VariationDevice
{
    friend struct Device;

private:
    hb_position_t get_x_delta(hb_font_t *font, const VariationStore &store) const
    {
        return font->em_scalef_x(get_delta(font, store));
    }

    hb_position_t get_y_delta(hb_font_t *font, const VariationStore &store) const
    {
        return font->em_scalef_y(get_delta(font, store));
    }

    void record_variation_index(hb_set_t *layout_variation_indices) const
    {
        unsigned var_idx = (outerIndex << 16) + innerIndex;
        layout_variation_indices->add(var_idx);
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        return_trace(c->check_struct(this));
    }

private:
    float get_delta(hb_font_t *font, const VariationStore &store) const
    {
        return store.get_delta(outerIndex, innerIndex, font->coords, font->num_coords);
    }

protected:
    HBUINT16 outerIndex;
    HBUINT16 innerIndex;
    HBUINT16 deltaFormat; /* Format identifier for this table: 0x0x8000 */
public:
    DEFINE_SIZE_STATIC(6);
};

struct DeviceHeader
{
protected:
    HBUINT16 reserved1;
    HBUINT16 reserved2;

public:
    HBUINT16 format; /* Format identifier */
public:
    DEFINE_SIZE_STATIC(6);
};

struct Device
{
    hb_position_t get_x_delta(hb_font_t *font, const VariationStore &store = Null(VariationStore)) const
    {
        switch (u.b.format) {
#ifndef HB_NO_HINTING
        case 1:
        case 2:
        case 3:
            return u.hinting.get_x_delta(font);
#endif
        case 0x8000:
            return u.variation.get_x_delta(font, store);
        default:
            return 0;
        }
    }
    hb_position_t get_y_delta(hb_font_t *font, const VariationStore &store = Null(VariationStore)) const
    {
        switch (u.b.format) {
        case 1:
        case 2:
        case 3:
#ifndef HB_NO_HINTING
            return u.hinting.get_y_delta(font);
#endif
        case 0x8000:
            return u.variation.get_y_delta(font, store);
        default:
            return 0;
        }
    }

    bool sanitize(hb_sanitize_context_t *c) const
    {
        TRACE_SANITIZE(this);
        if (!u.b.format.sanitize(c))
            return_trace(false);
        switch (u.b.format) {
#ifndef HB_NO_HINTING
        case 1:
        case 2:
        case 3:
            return_trace(u.hinting.sanitize(c));
#endif
        case 0x8000:
            return_trace(u.variation.sanitize(c));
        default:
            return_trace(true);
        }
    }

    void collect_variation_indices(hb_set_t *layout_variation_indices) const
    {
        switch (u.b.format) {
#ifndef HB_NO_HINTING
        case 1:
        case 2:
        case 3:
            return;
#endif
        case 0x8000:
            u.variation.record_variation_index(layout_variation_indices);
            return;
        default:
            return;
        }
    }

protected:
    union {
        DeviceHeader b;
        HintingDevice hinting;
        VariationDevice variation;
    } u;

public:
    DEFINE_SIZE_UNION(6, b);
};

} /* namespace OT */

#endif /* HB_OT_LAYOUT_COMMON_HH */