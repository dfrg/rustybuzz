/*
 * Copyright © 2009  Red Hat, Inc.
 * Copyright © 2012  Google, Inc.
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

#include "hb.hh"

#include "hb-machinery.hh"

#include "hb-ot.h"
#include "hb-face.hh"

/**
 * SECTION:hb-font
 * @title: hb-font
 * @short_description: Font objects
 * @include: hb.h
 *
 * Font objects represent a font face at a certain size and other
 * parameters (pixels per EM, points per EM, variation settings.)
 * Fonts are created from font faces, and are used as input to
 * rb_shape() among other things.
 **/

extern "C" {
RB_EXTERN rb_bool_t rb_font_metrics_get_position_common(rb_font_t *font, rb_tag_t tag, int *position);
}

enum rb_ot_metrics_tag_t {
    RB_FONT_METRICS_TAG_HORIZONTAL_ASCENDER = RB_TAG('h', 'a', 's', 'c'),
    RB_FONT_METRICS_TAG_HORIZONTAL_DESCENDER = RB_TAG('h', 'd', 's', 'c'),
    RB_FONT_METRICS_TAG_HORIZONTAL_LINE_GAP = RB_TAG('h', 'l', 'g', 'p'),
};

rb_bool_t rb_font_has_glyph(rb_font_t *font, rb_codepoint_t unicode)
{
    rb_codepoint_t glyph;
    return rb_font_get_nominal_glyph(font, unicode, &glyph);
}

rb_position_t rb_font_get_glyph_h_advance(rb_font_t *font, rb_codepoint_t glyph)
{
    rb_position_t ret;
    rb_font_get_glyph_h_advances(font, 1, &glyph, 0, &ret, 0);
    return ret;
}

void rb_font_get_glyph_h_advances(rb_font_t *font,
                                  unsigned int count,
                                  const rb_codepoint_t *first_glyph,
                                  unsigned glyph_stride,
                                  rb_position_t *first_advance,
                                  unsigned advance_stride)
{
    for (unsigned int i = 0; i < count; i++) {
        *first_advance = rb_font_get_advance(font, *first_glyph, 0);
        first_glyph = &StructAtOffsetUnaligned<rb_codepoint_t>(first_glyph, glyph_stride);
        first_advance = &StructAtOffsetUnaligned<rb_position_t>(first_advance, advance_stride);
    }
}

rb_position_t rb_font_get_glyph_v_advance(rb_font_t *font, rb_codepoint_t glyph)
{
    rb_position_t ret;
    rb_font_get_glyph_v_advances(font, 1, &glyph, 0, &ret, 0);
    return ret;
}

void rb_font_get_glyph_v_advances(rb_font_t *font,
                                  unsigned count,
                                  const rb_codepoint_t *first_glyph,
                                  unsigned glyph_stride,
                                  rb_position_t *first_advance,
                                  unsigned advance_stride)
{
    for (unsigned int i = 0; i < count; i++) {
        *first_advance = -rb_font_get_advance(font, *first_glyph, 1);
        first_glyph = &StructAtOffsetUnaligned<rb_codepoint_t>(first_glyph, glyph_stride);
        first_advance = &StructAtOffsetUnaligned<rb_position_t>(first_advance, advance_stride);
    }
}

rb_bool_t rb_font_get_glyph_contour_point_for_origin(rb_font_t *font,
                                                     rb_codepoint_t glyph,
                                                     unsigned int point_index,
                                                     rb_direction_t direction,
                                                     rb_position_t *x,
                                                     rb_position_t *y)
{
    *x = *y = 0;
    return false;
}

static rb_bool_t rb_font_get_h_extents(rb_font_t *font, rb_font_extents_t *extents)
{
    return rb_font_metrics_get_position_common(font, RB_FONT_METRICS_TAG_HORIZONTAL_ASCENDER, &extents->ascender) &&
           rb_font_metrics_get_position_common(font, RB_FONT_METRICS_TAG_HORIZONTAL_DESCENDER, &extents->descender) &&
           rb_font_metrics_get_position_common(font, RB_FONT_METRICS_TAG_HORIZONTAL_LINE_GAP, &extents->line_gap);
}

static rb_bool_t rb_font_get_glyph_h_origin(rb_font_t *font, rb_codepoint_t glyph, rb_position_t *x, rb_position_t *y)
{
    *x = *y = 0;
    return true;
}

static rb_bool_t rb_font_get_glyph_v_origin(rb_font_t *font, rb_codepoint_t glyph, rb_position_t *x, rb_position_t *y)
{
    *x = rb_font_get_glyph_h_advance(font, glyph) / 2;

    if (rb_font_has_vorg_data(font)) {
        *y = rb_font_get_y_origin(font, glyph);
        return true;
    }

    rb_glyph_extents_t extents = {0};
    rb_font_get_glyph_extents(font, glyph, &extents);

    rb_position_t tsb = rb_font_get_side_bearing(font, glyph, true);
    *y = extents.y_bearing + tsb;
    return true;
}

static void rb_font_get_h_extents_with_fallback(rb_font_t *font, rb_font_extents_t *extents)
{
    if (!rb_font_get_h_extents(font, extents)) {
        extents->ascender = rb_font_get_upem(font) * .8;
        extents->descender = extents->ascender - rb_font_get_upem(font);
        extents->line_gap = 0;
    }
}

static void rb_font_guess_v_origin_minus_h_origin(rb_font_t *font, rb_codepoint_t glyph, rb_position_t *x, rb_position_t *y)
{
    *x = rb_font_get_glyph_h_advance(font, glyph) / 2;

    /* TODO cache this somehow?! */
    rb_font_extents_t extents;
    rb_font_get_h_extents_with_fallback(font, &extents);
    *y = extents.ascender;
}

static void rb_font_get_glyph_v_origin_with_fallback(rb_font_t *font, rb_codepoint_t glyph, rb_position_t *x, rb_position_t *y)
{
    if (!rb_font_get_glyph_v_origin(font, glyph, x, y) && rb_font_get_glyph_h_origin(font, glyph, x, y)) {
        rb_position_t dx, dy;
        rb_font_guess_v_origin_minus_h_origin(font, glyph, &dx, &dy);
        *x += dx;
        *y += dy;
    }
}

void rb_font_subtract_glyph_v_origin(rb_font_t *font, rb_codepoint_t glyph, rb_position_t *x, rb_position_t *y)
{
    rb_position_t origin_x, origin_y;

    rb_font_get_glyph_v_origin_with_fallback(font, glyph, &origin_x, &origin_y);

    *x -= origin_x;
    *y -= origin_y;
}
