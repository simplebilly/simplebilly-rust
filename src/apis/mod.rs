use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String)
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod absence_api;
pub mod activity_api;
pub mod admin_api;
pub mod ai_api;
pub mod anlage_eks_api;
pub mod anlage_g_api;
pub mod anlage_s_api;
pub mod attachment_api;
pub mod attachment_version_api;
pub mod auth_api;
pub mod automations_api;
pub mod banking_api;
pub mod billing_api;
pub mod bom_api;
pub mod bookkeeping_api;
pub mod budgets_api;
pub mod compliance_training_api;
pub mod contact_api;
pub mod coupon_api;
pub mod create_sepa_direct_debit_api;
pub mod credit_note_api;
pub mod customer_api;
pub mod customer_communication_api;
pub mod customer_group_api;
pub mod datev_api;
pub mod datev_import_api;
pub mod declaration_api;
pub mod delivery_appointment_api;
pub mod delivery_date_api;
pub mod delivery_note_api;
pub mod down_payment_invoice_api;
pub mod ebilanz_api;
pub mod email_template_api;
pub mod emissions_api;
pub mod employee_api;
pub mod euer_api;
pub mod event_subscription_api;
pub mod fristen_api;
pub mod gdpr_api;
pub mod generate_qrcode_api;
pub mod generate_xrechnung_api;
pub mod gewerbesteuer_api;
pub mod gewinnverwendung_api;
pub mod gez_api;
pub mod gobd_export_api;
pub mod goods_receipt_api;
pub mod group_figure_api;
pub mod import_runner_api;
pub mod institute_api;
pub mod institute_profile_api;
pub mod inventory_count_api;
pub mod inventory_value_api;
pub mod invoice_api;
pub mod job_application_api;
pub mod job_posting_api;
pub mod konzern_api;
pub mod kosten_vorschau_api;
pub mod kst_api;
pub mod kyc_record_api;
pub mod lead_api;
pub mod legal_document_api;
pub mod list_open_items_api;
pub mod marketplace_api_api;
pub mod notifications_api;
pub mod offenlegung_api;
pub mod onlineshop_api;
pub mod order_api;
pub mod order_confirmation_api;
pub mod oss_report_api;
pub mod packing_api;
pub mod participation_api;
pub mod paygap_api;
pub mod payment_api;
pub mod payment_condition_api;
pub mod payment_gateway_api;
pub mod payroll_api;
pub mod peppol_api;
pub mod plausibility_api;
pub mod pos_api;
pub mod posting_category_api;
pub mod price_tier_api;
pub mod product_api;
pub mod product_attribute_api;
pub mod product_category_api;
pub mod product_variant_api;
pub mod production_order_api;
pub mod proforma_invoice_api;
pub mod propose_assignments_api;
pub mod public_returns_api;
pub mod purchase_order_api;
pub mod quotation_api;
pub mod recurring_template_api;
pub mod reorder_proposal_api;
pub mod replenishment_api;
pub mod reports_api;
pub mod return_order_api;
pub mod rfq_api;
pub mod search_api;
pub mod service_assignment_api;
pub mod service_job_api;
pub mod shareholder_api;
pub mod shipment_api;
pub mod shipping_api;
pub mod shipping_rule_api;
pub mod shipping_threshold_api;
pub mod shop_api;
pub mod silent_partner_api;
pub mod stille_api;
pub mod stock_movement_api;
pub mod stock_transfer_api;
pub mod suitability_api;
pub mod supplier_condition_api;
pub mod supplier_invoice_api;
pub mod support_channel_api;
pub mod support_ticket_api;
pub mod tax_api;
pub mod tenant_settings_api;
pub mod ticket_message_api;
pub mod time_entries_api;
pub mod training_assignment_api;
pub mod trainings_api;
pub mod user_api;
pub mod user_management_api;
pub mod ustva_api;
pub mod voucher_api;
pub mod warehouse_api;
pub mod warehouse_stock_api;
pub mod webhooks_api;
pub mod workflows_api;
pub mod zugferd_api;

pub mod configuration;
