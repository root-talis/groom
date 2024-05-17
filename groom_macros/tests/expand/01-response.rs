//! This is expansion preview for #[Response] annotation.
//! Each case is put into its own `mod` to make it easier to inspect expansion result.

#[macro_use]
extern crate groom_macros;

mod no_content_type {
    #[Response()]
    pub enum RespJsonResponse {
        #[Response(code = 202)]
        Accepted,

        #[Response(code = 404)]
        NotFound,
    }
}

mod plaintext_only {
    #[Response(format(plain_text))]
    pub enum RespPlaintextResponse {
        #[Response()]
        Ok(String),

        #[Response(code = 404)]
        NotFound,
    }
}

mod html_only {
    use groom::response::html_format;
    use groom::response::HtmlFormat;

    #[DTO(response)]
    pub struct Struct {
        success: bool
    }

    html_format!(Struct, self {
        if self.success {
            "<span style=\"color: #a3be8c;\">success</span>"
        } else {
            "<span style=\"color: #bf616a;\">error</span>"
        }
    });

    #[Response(format(html))]
    pub enum RespHtmlResponse {
        #[Response()]
        Ok(Struct),

        #[Response(code = 404)]
        NotFound,
    }
}

mod json_only {
    #[DTO(response)]
    pub struct StructJson {
        success: bool
    }

    #[Response(format(json))]
    pub enum RespJsonResponse {
        #[Response()]
        Ok(StructJson),

        #[Response(code = 404)]
        NotFound,
    }
}


mod multiple_content_types {
    use groom::response::html_format;
    use groom::response::HtmlFormat;

    #[DTO(response)]
    pub struct Struct {
        success: bool
    }

    html_format!(Struct, self {
        if self.success {
            "<span style=\"color: #a3be8c;\">success</span>"
        } else {
            "<span style=\"color: #bf616a;\">error</span>"
        }
    });

    #[Response(format(json, html, plain_text), default_format="json")]
    pub enum RespMultipleTypesResponse {
        #[Response()]
        Ok(Struct),

        #[Response(code = 404)]
        NotFound,
    }
}
