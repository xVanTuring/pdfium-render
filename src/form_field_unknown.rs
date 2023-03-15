//! Defines the [PdfFormUnknownField] struct, exposing functionality related to a single
//! form field of type `PdfFormFieldType::Unknown`.
//!
use crate::bindgen::{FPDF_ANNOTATION, FPDF_FORMHANDLE};
use crate::bindings::PdfiumLibraryBindings;
use crate::form_field_private::internal::PdfFormFieldPrivate;

pub struct PdfFormUnknownField<'a> {
    form_handle: FPDF_FORMHANDLE,
    annotation_handle: FPDF_ANNOTATION,
    bindings: &'a dyn PdfiumLibraryBindings,
}

impl<'a> PdfFormUnknownField<'a> {
    pub(crate) fn from_pdfium(
        form_handle: FPDF_FORMHANDLE,
        annotation_handle: FPDF_ANNOTATION,
        bindings: &'a dyn PdfiumLibraryBindings,
    ) -> Self {
        PdfFormUnknownField {
            form_handle,
            annotation_handle,
            bindings,
        }
    }

    /// Returns the [PdfiumLibraryBindings] used by this [PdfFormUnknownField] object.
    #[inline]
    pub fn bindings(&self) -> &'a dyn PdfiumLibraryBindings {
        self.bindings
    }
}

impl<'a> PdfFormFieldPrivate<'a> for PdfFormUnknownField<'a> {
    #[inline]
    fn form_handle(&self) -> &FPDF_FORMHANDLE {
        &self.form_handle
    }

    #[inline]
    fn annotation_handle(&self) -> &FPDF_ANNOTATION {
        &self.annotation_handle
    }

    #[inline]
    fn bindings(&self) -> &dyn PdfiumLibraryBindings {
        self.bindings
    }
}
