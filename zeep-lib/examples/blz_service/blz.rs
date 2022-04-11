//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.5
//!

#![allow(dead_code)]
#![allow(unused_imports)]
use log::{debug, warn};
use std::io::{Read, Write};
use yaserde_derive::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Header {}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    rename = "Fault",
    namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
    prefix = "soapenv"
)]
pub struct SoapFault {
    #[yaserde(rename = "faultcode", default)]
    pub fault_code: Option<String>,
    #[yaserde(rename = "faultstring", default)]
    pub fault_string: Option<String>,
}
pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;

pub mod messages {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "getBank")]
    pub struct GetBank {
        #[yaserde(flatten, default)]
        pub parameters: types::GetBank,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(rename = "getBankResponse")]
    pub struct GetBankResponse {
        #[yaserde(flatten, default)]
        pub parameters: types::GetBankResponse,
    }
}

pub mod types {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    pub type GetBank = GetBankType;

    pub type GetBankResponse = GetBankResponseType;

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "getBankType",
        namespace = "tns: http://thomas-bayer.com/blz/",
        prefix = "tns"
    )]
    pub struct GetBankType {
        #[yaserde(rename = "blz", prefix = "tns", default)]
        pub blz: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "getBankResponseType",
        namespace = "tns: http://thomas-bayer.com/blz/",
        prefix = "tns"
    )]
    pub struct GetBankResponseType {
        #[yaserde(rename = "details", prefix = "tns", default)]
        pub details: DetailsType,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        rename = "detailsType",
        namespace = "tns: http://thomas-bayer.com/blz/",
        prefix = "tns"
    )]
    pub struct DetailsType {
        #[yaserde(rename = "bezeichnung", prefix = "tns", default)]
        pub bezeichnung: Option<String>,
        #[yaserde(rename = "bic", prefix = "tns", default)]
        pub bic: Option<String>,
        #[yaserde(rename = "ort", prefix = "tns", default)]
        pub ort: Option<String>,
        #[yaserde(rename = "plz", prefix = "tns", default)]
        pub plz: Option<String>,
    }
}

pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    pub type GetBank = messages::GetBank;

    pub type GetBankResponse = messages::GetBankResponse;

    #[async_trait]
    pub trait BlzservicePortType {
        async fn get_bank(&self, get_bank: GetBank) -> Result<GetBankResponse, Option<SoapFault>>;
    }
}

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};

    impl BlzserviceSOAP11Binding {
        async fn send_soap_request<T: YaSerialize>(
            &self,
            request: &T,
            action: &str,
        ) -> SoapResponse {
            let body = to_string(request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);
            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", action);
            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(
                    credentials.0.to_string(),
                    Option::Some(credentials.1.to_string()),
                );
            }
            let res = req.send().await?;
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            Ok((status, txt))
        }
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetBank {
        #[yaserde(rename = "GetBank", default)]
        pub body: ports::GetBank,
        #[yaserde(attribute)]
        pub xmlns: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetBankSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetBank,
    }

    impl GetBankSoapEnvelope {
        pub fn new(body: SoapGetBank) -> Self {
            GetBankSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::Some("http://thomas-bayer.com/blz/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    pub struct SoapGetBankResponse {
        #[yaserde(rename = "GetBankResponse", default)]
        pub body: ports::GetBankResponse,
        #[yaserde(rename = "Fault", default)]
        pub fault: Option<SoapFault>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize)]
    #[yaserde(
        rename = "Envelope",
        namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
        prefix = "soapenv"
    )]
    pub struct GetBankResponseSoapEnvelope {
        #[yaserde(rename = "encodingStyle", prefix = "soapenv", attribute)]
        pub encoding_style: String,
        #[yaserde(rename = "tns", prefix = "xmlns", attribute)]
        pub tnsattr: Option<String>,
        #[yaserde(rename = "urn", prefix = "xmlns", attribute)]
        pub urnattr: Option<String>,
        #[yaserde(rename = "xsi", prefix = "xmlns", attribute)]
        pub xsiattr: Option<String>,
        #[yaserde(rename = "Header", prefix = "soapenv")]
        pub header: Option<Header>,
        #[yaserde(rename = "Body", prefix = "soapenv")]
        pub body: SoapGetBankResponse,
    }

    impl GetBankResponseSoapEnvelope {
        pub fn new(body: SoapGetBankResponse) -> Self {
            GetBankResponseSoapEnvelope {
                encoding_style: SOAP_ENCODING.to_string(),
                tnsattr: Option::Some("http://thomas-bayer.com/blz/".to_string()),
                body,
                urnattr: None,
                xsiattr: None,
                header: None,
            }
        }
    }

    impl Default for BlzserviceSOAP11Binding {
        fn default() -> Self {
            BlzserviceSOAP11Binding {
                client: reqwest::Client::new(),
                url: "http://thomas-bayer.com/blz/".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl BlzserviceSOAP11Binding {
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            BlzserviceSOAP11Binding {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    pub struct BlzserviceSOAP11Binding {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }
    #[async_trait]
    impl ports::BlzservicePortType for BlzserviceSOAP11Binding {
        async fn get_bank(
            &self,
            get_bank: ports::GetBank,
        ) -> Result<ports::GetBankResponse, Option<SoapFault>> {
            let __request = GetBankSoapEnvelope::new(SoapGetBank {
                body: get_bank,
                xmlns: Option::Some("http://thomas-bayer.com/blz/".to_string()),
            });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: GetBankResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
    }

    impl BlzserviceSOAP12Binding {
        async fn send_soap_request<T: YaSerialize>(
            &self,
            request: &T,
            action: &str,
        ) -> SoapResponse {
            let body = to_string(request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);
            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", action);
            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(
                    credentials.0.to_string(),
                    Option::Some(credentials.1.to_string()),
                );
            }
            let res = req.send().await?;
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            Ok((status, txt))
        }
    }
    impl Default for BlzserviceSOAP12Binding {
        fn default() -> Self {
            BlzserviceSOAP12Binding {
                client: reqwest::Client::new(),
                url: "http://thomas-bayer.com/blz/".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl BlzserviceSOAP12Binding {
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            BlzserviceSOAP12Binding {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    pub struct BlzserviceSOAP12Binding {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }
    #[async_trait]
    impl ports::BlzservicePortType for BlzserviceSOAP12Binding {
        async fn get_bank(
            &self,
            get_bank: ports::GetBank,
        ) -> Result<ports::GetBankResponse, Option<SoapFault>> {
            let __request = GetBankSoapEnvelope::new(SoapGetBank {
                body: get_bank,
                xmlns: Option::Some("http://thomas-bayer.com/blz/".to_string()),
            });

            let (status, response) =
                self.send_soap_request(&__request, "")
                    .await
                    .map_err(|err| {
                        warn!("Failed to send SOAP request: {:?}", err);
                        None
                    })?;

            let r: GetBankResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
    }

    impl BlzserviceHttpBinding {
        async fn send_soap_request<T: YaSerialize>(
            &self,
            request: &T,
            action: &str,
        ) -> SoapResponse {
            let body = to_string(request).expect("failed to generate xml");
            debug!("SOAP Request: {}", body);
            let mut req = self
                .client
                .post(&self.url)
                .body(body)
                .header("Content-Type", "text/xml")
                .header("Soapaction", action);
            if let Some(credentials) = &self.credentials {
                req = req.basic_auth(
                    credentials.0.to_string(),
                    Option::Some(credentials.1.to_string()),
                );
            }
            let res = req.send().await?;
            let status = res.status();
            debug!("SOAP Status: {}", status);
            let txt = res.text().await.unwrap_or_default();
            debug!("SOAP Response: {}", txt);
            Ok((status, txt))
        }
    }
    impl Default for BlzserviceHttpBinding {
        fn default() -> Self {
            BlzserviceHttpBinding {
                client: reqwest::Client::new(),
                url: "http://thomas-bayer.com/blz/".to_string(),
                credentials: Option::None,
            }
        }
    }
    impl BlzserviceHttpBinding {
        pub fn new(url: &str, credentials: Option<(String, String)>) -> Self {
            BlzserviceHttpBinding {
                client: reqwest::Client::new(),
                url: url.to_string(),
                credentials,
            }
        }
    }
    pub struct BlzserviceHttpBinding {
        client: reqwest::Client,
        url: String,
        credentials: Option<(String, String)>,
    }
    #[async_trait]
    impl ports::BlzservicePortType for BlzserviceHttpBinding {
        async fn get_bank(
            &self,
            get_bank: ports::GetBank,
        ) -> Result<ports::GetBankResponse, Option<SoapFault>> {
            let __request = GetBankSoapEnvelope::new(SoapGetBank {
                body: get_bank,
                xmlns: Option::Some("http://thomas-bayer.com/blz/".to_string()),
            });

            let (status, response) = self
                .send_soap_request(&__request, "http://thomas-bayer.com/blz//getBank")
                .await
                .map_err(|err| {
                    warn!("Failed to send SOAP request: {:?}", err);
                    None
                })?;

            let r: GetBankResponseSoapEnvelope = from_str(&response).map_err(|err| {
                warn!("Failed to unmarshal SOAP response: {:?}", err);
                None
            })?;
            if status.is_success() {
                Ok(r.body.body)
            } else {
                Err(r.body.fault)
            }
        }
    }
}

pub mod services {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    pub struct Blzservice {}
    impl Blzservice {
        pub fn new_client(
            credentials: Option<(String, String)>,
        ) -> bindings::BlzserviceSOAP11Binding {
            bindings::BlzserviceSOAP11Binding::new(
                "http://www.thomas-bayer.com/axis2/services/BLZService",
                credentials,
            )
        }
    }
}
