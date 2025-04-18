// Dimensions taken from
//
// https://www.six-group.com/dam/download/banking-services/standardization/qr-bill/style-guide-qr-bill-en.pdf
//
// which is stored locally in
//
//   qr-standard-docs/style-guide-qr-bill-en.pdf
//
// The dimensions of the blank rectangles are on page 7, the other dimensions on
// page 15.



// 3.4 // Fonts and font sizes
//
// Only the sans-serif fonts Arial, Frutiger, Helvetica and Liberation Sans are
// permitted in black. Text must not be in italics nor underlined.
//
// The font size for headings and their associated values on the payment part
// must be at least 6 pt, and maximum 10 pt. Headings in the "Amount" and
// "Details" sections must always be the same size. They should be printed in
// bold and 2 pt smaller than the font size for their associated values. The
// recommended font size for headings is 8 pt and for the associated values 10
// pt. The only exception, in font size 11 pt (bold), is the title "Payment
// part".
//
// When filling in the "Alternative procedures" element, the font size is 7 pt,
// with the name of the alternative procedure printed in bold type.
//
// The "Ultimate creditor" element is intended for use in the future but will
// not be used for the QR-bill and should therefore not be filled in. If
// approval is given for the field to be filled in, the font size is expected to
// be 7 pt with the designation in bold type.
//
// The font sizes for the receipt are 6 pt for the headings (bold) and 8 pt for
// the associated values. The exception, in font size 11 pt (bold), is the title
// "Receipt".



// TODO replace this with Length(f64), but then the mm/pt constructors become
// non-const functions and the we cannot make the RECEIPT/PAYMENT consts
#[derive(Debug, Copy, Clone)]
pub struct Length {
    in_svg_uu: f64,
}

pub const MM_TO_UU: f64 = 3.543307;
pub const PT_TO_UU: f64 = PT_TO_MM * MM_TO_UU;
pub const PT_TO_MM: f64 = 0.3527777778;

impl Length {
    pub fn as_mm(self) -> f64 { self.in_svg_uu / MM_TO_UU }
    pub fn as_pt(self) -> f64 { self.in_svg_uu / PT_TO_UU }
    pub fn as_uu(self) -> f64 { self.in_svg_uu            }
    pub fn mm(mm: f64) -> Self { Self { in_svg_uu: mm * MM_TO_UU } }
    pub fn pt(pt: f64) -> Self { Self { in_svg_uu: pt * PT_TO_UU } }
    pub fn uu(uu: f64) -> Self { Self { in_svg_uu: uu            } }
}

impl From<Length> for svg::node::Value {
    fn from(Length { in_svg_uu }: Length) -> Self {
        in_svg_uu.into()
    }
}

impl std::ops::AddAssign for Length {
    fn add_assign(&mut self, rhs: Self) {
        self.in_svg_uu += rhs.in_svg_uu;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Xy { pub x: Length, pub y: Length }

impl Xy {
    pub fn mm(left: f64, top: f64) -> Self {
        Self { x: Length::mm(left), y: Length::mm(top) }
    }
}

/// Information about positions an sizes of elements to be rendered on one part
/// (receipt or payment) of a QRBill.
pub struct Dimensions {
    pub section: Sections,
    pub font: Fonts,
    // Dimensions of blank rectangles
    pub blank_payable:  Xy,
    pub blank_amount:   Xy,
    pub max_chars_line: usize,
}

const RCT_X: f64 =   5.0; // mm x-position of RECEIPT part sections
const PAY_X: f64 =  67.0; // mm x-position of PAYMENT part sections except INFORMATION
const INF_X: f64 = 118.0; // mm x-position of INFORMATION section in PAYMENT part
const ACC_E: f64 =  57.0; // mm x-position of RHS of ACCEPTANCE POINT section

pub fn receipt() -> Dimensions { Dimensions {
    section: Sections {
        title:            Xy::mm(RCT_X,  5.0),
        information:      Xy::mm(RCT_X, 12.0),
        amount:           Xy::mm(RCT_X, 68.0),
        acceptance:  Some(Xy::mm(ACC_E, 82.0)),
        qr_code:     None,
        alt_proc:    None,
    },

    // The font sizes for the receipt are 6 pt for the headings (bold) and 8 pt
    // for the associated values. The exception, in font size 11 pt (bold), is
    // the title "Receipt".
    font: Fonts {           //    size  line-spacing
        title:              font( 11.0, 11.0), // bold
        heading:            font(  6.0,  9.0), // bold
        value:              font(  8.0,  9.0),
        amount:             font(  8.0, 11.0),
        acceptance_pt: Some(font(  6.0,  8.0)), // bold
        alt_proc:      None,
    },

    blank_payable: Xy::mm( 52.0, 20.0),
    blank_amount:  Xy::mm( 30.0, 10.0),

    max_chars_line: 38,
}}

pub fn payment() -> Dimensions { Dimensions {
    section: Sections {
        title:            Xy::mm(PAY_X,  5.0),
        information:      Xy::mm(INF_X,  5.0),
        amount:           Xy::mm(PAY_X, 68.0),
        acceptance:  None,
        qr_code:     Some(Xy::mm(PAY_X, 17.0)),
        alt_proc:    Some(Xy::mm(PAY_X, 90.0)),
    },

    // The font size for headings and their associated values on the payment
    // part must be at least 6 pt, and maximum 10 pt.
    //
    // Headings in the "Amount" and "Details" sections must always be the same
    // size. They should be printed in bold and 2 pt smaller than the font size
    // for their associated values.
    //
    // The recommended font size for headings is 8 pt and for the associated
    // values 10 pt.
    //
    // The only exception, in font size 11 pt (bold), is the title "Payment
    // part".
    //
    // When filling in the "Alternative procedures" element, the font size is 7
    // pt, with the name of the alternative procedure printed in bold type.
    font: Fonts {           //    size  line-spacing
        title:              font( 11.0, 11.0), // bold
        heading:            font(  8.0, 11.0), // bold
        value:              font( 10.0, 11.0),
        amount:             font( 10.0, 13.0),
        acceptance_pt: None,
        alt_proc:      Some(font(  7.0,  8.0)), // bold & normal
    },

    blank_payable: Xy::mm( 65.0, 25.0),
    blank_amount:  Xy::mm( 40.0, 15.0),

    max_chars_line: 72,
}}

pub struct Sections {
    pub title:               Xy,
    pub information:         Xy,
    pub amount:              Xy,
    pub acceptance:   Option<Xy>,
    pub qr_code:      Option<Xy>,
    pub alt_proc:     Option<Xy>,
}

pub struct Fonts {
    pub title:                Font,
    pub heading:              Font,
    pub value:                Font,
    pub amount:               Font,
    pub acceptance_pt: Option<Font>,
    pub alt_proc:      Option<Font>,
}

#[derive(Debug, Clone, Copy)]
pub struct Font { pub (crate) size: Length, pub (crate) line_spacing: Length }

fn font(size_in_pt: f64, line_spacing_in_pt: f64) -> Font {
    Font {
        size:         Length::pt(size_in_pt),
        line_spacing: Length::pt(line_spacing_in_pt),
    }
}

pub mod blank_rectangle {
    use super::*;
    pub fn line_length() -> Length { Length::mm(3.0 ) }
    pub fn line_width () -> Length { Length::pt(0.75) }
    
}

pub fn make_svg_styles() -> String {
    let r = receipt().font;
    let p = payment().font;

    let r_titl = r.title                 .size.as_pt();
    let r_head = r.heading               .size.as_pt();
    let r_valu = r.value                 .size.as_pt();
    let r_acpt = r.acceptance_pt.unwrap().size.as_pt();

    let p_titl = p.title                 .size.as_pt();
    let p_head = p.heading               .size.as_pt();
    let p_valu = p.value                 .size.as_pt();
    let p_altp = p.alt_proc     .unwrap().size.as_pt();

    format!("
    text {{
         font-family: Arial, Helvetica, Frutiger, \"Liberation Sans\", sans-serif;
    }}
    .r-title         {{ font-size: {r_titl:2.0}pt; font-weight: bold; }}
    .r-heading       {{ font-size: {r_head:2.0}pt; font-weight: bold; }}
    .r-value         {{ font-size: {r_valu:2.0}pt;                    }}
    .r-acceptance-pt {{ font-size: {r_acpt:2.0}pt; font-weight: bold; }}

    .p-title         {{ font-size: {p_titl:2.0}pt; font-weight: bold; }}
    .p-heading       {{ font-size: {p_head:2.0}pt; font-weight: bold; }}
    .p-value         {{ font-size: {p_valu:2.0}pt;                    }}
    .p-alt-proc      {{ font-size: {p_altp:2.0}pt; font-weight: bold; }}
    }}
")
}
