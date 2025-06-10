use serde::{Deserialize, Serialize};

/// Request parameters for creating a new user.
///
/// Contains the required and optional fields for user creation.
/// Email is the only required field.
#[derive(Debug, Serialize)]
pub struct CreateUserRequest<'a> {
    /// User's email address (required)
    pub email: &'a str,
    /// User's first name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,
    /// User's last name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
    /// User's personal name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_name: Option<&'a str>,
    /// User's phone number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// User's custom parameters as JSON string (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_params: Option<&'a str>,
}

impl<'a> CreateUserRequest<'a> {
    /// Create a new user creation request with the required email field.
    pub fn new(email: &'a str) -> Self {
        Self {
            email,
            first_name: None,
            last_name: None,
            personal_name: None,
            phone: None,
            custom_params: None,
        }
    }

    /// Set the first name for the user.
    pub fn with_first_name(mut self, first_name: &'a str) -> Self {
        self.first_name = Some(first_name);
        self
    }

    /// Set the last name for the user.
    pub fn with_last_name(mut self, last_name: &'a str) -> Self {
        self.last_name = Some(last_name);
        self
    }

    /// Set the personal name for the user.
    pub fn with_personal_name(mut self, personal_name: &'a str) -> Self {
        self.personal_name = Some(personal_name);
        self
    }

    /// Set the phone number for the user.
    pub fn with_phone(mut self, phone: &'a str) -> Self {
        self.phone = Some(phone);
        self
    }

    /// Set custom parameters for the user (as JSON string).
    pub fn with_custom_params(mut self, custom_params: &'a str) -> Self {
        self.custom_params = Some(custom_params);
        self
    }
}

/// Request parameters for updating an existing user.
///
/// Contains the UID of the user to update and optional fields to modify.
#[derive(Debug, Serialize)]
pub struct UpdateUserRequest<'a> {
    /// User's unique identifier (required)
    pub uid: &'a str,
    /// User's email address (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    /// User's first name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<&'a str>,
    /// User's last name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<&'a str>,
    /// User's personal name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_name: Option<&'a str>,
    /// User's phone number (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    /// User's custom parameters as JSON string (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_params: Option<&'a str>,
}

impl<'a> UpdateUserRequest<'a> {
    /// Create a new user update request with the required UID.
    pub fn new(uid: &'a str) -> Self {
        Self {
            uid,
            email: None,
            first_name: None,
            last_name: None,
            personal_name: None,
            phone: None,
            custom_params: None,
        }
    }

    /// Set the email for the user.
    pub fn with_email(mut self, email: &'a str) -> Self {
        self.email = Some(email);
        self
    }

    /// Set the first name for the user.
    pub fn with_first_name(mut self, first_name: &'a str) -> Self {
        self.first_name = Some(first_name);
        self
    }

    /// Set the last name for the user.
    pub fn with_last_name(mut self, last_name: &'a str) -> Self {
        self.last_name = Some(last_name);
        self
    }

    /// Set the personal name for the user.
    pub fn with_personal_name(mut self, personal_name: &'a str) -> Self {
        self.personal_name = Some(personal_name);
        self
    }

    /// Set the phone number for the user.
    pub fn with_phone(mut self, phone: &'a str) -> Self {
        self.phone = Some(phone);
        self
    }

    /// Set custom parameters for the user (as JSON string).
    pub fn with_custom_params(mut self, custom_params: &'a str) -> Self {
        self.custom_params = Some(custom_params);
        self
    }
}

/// Request parameters for listing users with filtering and pagination.
#[derive(Debug, Serialize, Default)]
pub struct ListUserRequest<'a> {
    /// Maximum number of results to return (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<usize>,
    /// Offset for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    /// Search query string (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<&'a str>,
    /// Filter by user status (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<&'a str>,
    /// Order by field (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<&'a str>,
    /// Order direction (asc/desc) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_direction: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_fields: Option<CustomFieldQuery>,
}

impl<'a> ListUserRequest<'a> {
    /// Create a new user list request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the limit for pagination.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the offset for pagination.
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Set the search query string.
    pub fn with_query(mut self, q: &'a str) -> Self {
        self.q = Some(q);
        self
    }

    /// Set the status filter.
    pub fn with_status(mut self, status: &'a str) -> Self {
        self.status = Some(status);
        self
    }

    /// Set the order by field.
    pub fn with_order_by(mut self, order_by: &'a str) -> Self {
        self.order_by = Some(order_by);
        self
    }

    /// Set the order direction.
    pub fn with_order_direction(mut self, order_direction: &'a str) -> Self {
        self.order_direction = Some(order_direction);
        self
    }
}

/// Request parameters for searching users.
#[derive(Debug, Serialize, Default)]
pub struct SearchUserRequest<'a> {
    /// Email to search for (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<&'a str>,
    /// UID to search for (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<&'a str>,
    /// Maximum number of results to return (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    /// Offset for pagination (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_fields: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum Source {
    VX,
    CF,
}
#[derive(Debug, Serialize, Default)]
pub struct CustomFieldQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Condition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data_type: Option<CustomFieldDataType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    response_time: Option<ResponseTimeQuery>,
    #[serde(rename = "fieldTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    field_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_name: Option<String>,
}

impl CustomFieldQuery {
    /// Create a new custom field query.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the condition for the custom field query.
    pub fn with_condition(mut self, condition: Condition) -> Self {
        self.condition = Some(condition);
        self
    }

    /// Set the data type for the custom field query.
    pub fn with_data_type(mut self, data_type: CustomFieldDataType) -> Self {
        self.data_type = Some(data_type);
        self
    }

    /// Set the response time query for the custom field query.
    pub fn with_response_time(mut self, response_time: ResponseTimeQuery) -> Self {
        self.response_time = Some(response_time);
        self
    }

    /// Set the field title for the custom field query.
    pub fn with_field_title(mut self, field_title: String) -> Self {
        self.field_title = Some(field_title);
        self
    }

    /// Set the enabled flag for the custom field query.
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    /// Set the field name for the custom field query.
    pub fn with_field_name(mut self, field_name: &str) -> Self {
        self.field_name = Some(field_name.to_string());
        self
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ResponseTimeQuery {
    #[serde(rename = "BETWEEN")]
    Between { more: String, less: String },
    #[serde(rename = "CURRENT")]
    Current,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Condition {
    #[serde(rename = "EQUAL")]
    Equal {
        #[serde(rename = "optionsEqual")]
        options_equal: EqExpr,
    },
}

impl Condition {
    pub fn string_equals(id: u32, value: &str) -> Self {
        Self::Equal {
            options_equal: EqExpr::String {
                id,
                value: value.to_string(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum EqExpr {
    String { id: u32, value: String },
}

impl EqExpr {
    pub fn string(id: u32, value: &str) -> Self {
        Self::String {
            id,
            value: value.to_string(),
        }
    }
}
impl<'a> SearchUserRequest<'a> {
    /// Create a new user search request.
    pub fn new() -> Self {
        Self::default()
    }

    /// Search by email.
    pub fn with_email(mut self, email: &'a str) -> Self {
        self.email = Some(email);
        self
    }

    /// Search by source.
    pub fn with_source(mut self, source: Source) -> Self {
        self.source = Some(source);
        self
    }
    pub fn with_custom_field_queries(mut self, custom_field_queries: &[CustomFieldQuery]) -> Self {
        let value = serde_json::to_string(custom_field_queries).expect("OK");
        self.custom_fields = Some(value);
        self
    }

    /// Search by UID.
    pub fn with_uid(mut self, uid: &'a str) -> Self {
        self.uid = Some(uid);
        self
    }

    /// Set the limit for pagination.
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the offset for pagination.
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }
}

/// Response wrapper for single user operations.
#[derive(Debug, Deserialize, Clone)]
pub(super) struct UserResult {
    /// The user data - Piano API can return with different field names
    #[serde(alias = "User", alias = "user")]
    pub user: User,
}

/// Response for user list operations.
#[derive(Debug, Deserialize, Clone)]
pub struct ListUserResult {
    /// Array of users
    #[serde(alias = "Users", alias = "users")]
    pub users: Vec<User>,
}

/// Represents a Piano platform user.
///
/// Contains user information including personal details and platform-specific data.
#[derive(Debug, Deserialize, Clone)]
pub struct User {
    /// User's unique identifier
    uid: String,
    /// User's email address
    email: String,
    /// User's first name (optional)
    #[serde(default)]
    first_name: Option<String>,
    /// User's last name (optional)
    #[serde(default)]
    last_name: Option<String>,
    /// User's personal name (optional)
    #[serde(default)]
    personal_name: Option<String>,
    /// User's display name (optional)
    #[serde(default)]
    display_name: Option<String>,
    /// User's phone number (optional)
    #[serde(default)]
    phone: Option<String>,
    /// Timestamp when user was created (UNIX timestamp)
    create_date: i64,
    /// Timestamp when user was last updated (UNIX timestamp)
    #[serde(default)]
    update_date: Option<i64>,
    /// User's current status
    #[serde(default)]
    status: Option<String>,
    /// Custom parameters as JSON string (optional)
    #[serde(default)]
    custom_fields: Vec<CustomField>,
    #[serde(default)]
    reset_password_email_sent: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CustomField {
    archived: bool,
    attribute: CustomFieldAttribute,
    #[serde(default)]
    comment: String,
    /// Example: "2024-11-06T02:54:24.000+00:00",
    created: Option<String>,
    #[serde(rename = "dataType")]
    data_type: CustomFieldDataType,
    #[serde(rename = "defaultSortOrder")]
    default_sort_order: Option<usize>,
    #[serde(rename = "sortOrder")]
    sort_order: Option<usize>,
    editable: bool,
    #[serde(default)]
    #[serde(rename = "favouriteOptions")]
    favourite_options: Option<Vec<String>>,
    #[serde(default)]
    options: Vec<String>,
    #[serde(rename = "emailCreator")]
    email_creator: Option<String>,
    #[serde(rename = "fieldName")]
    field_name: String,
    title: String,
    tooltip: Option<Tooltip>,
    value: Option<String>,
}

impl CustomField {
    /// Get whether the custom field is archived.
    pub fn archived(&self) -> bool {
        self.archived
    }

    pub fn as_vec(&self) -> Option<Vec<String>> {
        match self.data_type() {
            CustomFieldDataType::SingleSelectList => match &self.value {
                None => Some(vec![]),
                Some(str) => Some(
                    str.trim_start_matches("[")
                        .trim_end_matches("]")
                        .split(",")
                        .into_iter()
                        .map(Self::unquote_str)
                        .collect::<Vec<_>>(),
                ),
            },
            CustomFieldDataType::Boolean
            | CustomFieldDataType::ISODate
            | CustomFieldDataType::Text => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self.data_type() {
            CustomFieldDataType::Boolean => self.value().unwrap_or_default().parse::<bool>().ok(),
            CustomFieldDataType::ISODate
            | CustomFieldDataType::SingleSelectList
            | CustomFieldDataType::Text => None,
        }
    }
    fn unquote_str(s: &str) -> String {
        s.trim_start_matches("\"")
            .trim_end_matches("\"")
            .to_string()
    }

    /// Get the custom field attribute.
    pub fn attribute(&self) -> &CustomFieldAttribute {
        &self.attribute
    }

    /// Get the custom field comment.
    pub fn comment(&self) -> &str {
        &self.comment
    }

    /// Get the custom field creation timestamp.
    pub fn created(&self) -> Option<&str> {
        self.created.as_deref()
    }

    /// Get the custom field data type.
    pub fn data_type(&self) -> &CustomFieldDataType {
        &self.data_type
    }

    /// Get the default sort order.
    pub fn default_sort_order(&self) -> Option<usize> {
        self.default_sort_order
    }

    /// Get the sort order.
    pub fn sort_order(&self) -> Option<usize> {
        self.sort_order
    }

    /// Get whether the custom field is editable.
    pub fn editable(&self) -> bool {
        self.editable
    }

    /// Get the favourite options.
    pub fn favourite_options(&self) -> Option<&[String]> {
        self.favourite_options.as_deref()
    }

    /// Get the options.
    pub fn options(&self) -> &[String] {
        &self.options
    }

    /// Get the email creator.
    pub fn email_creator(&self) -> Option<&str> {
        self.email_creator.as_deref()
    }

    /// Get the field name.
    pub fn field_name(&self) -> &str {
        &self.field_name
    }

    /// Get the title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Get the tooltip.
    pub fn tooltip(&self) -> Option<&Tooltip> {
        self.tooltip.as_ref()
    }

    /// Get the value.
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct CustomFieldAttribute {
    #[serde(default)]
    multiline: Option<bool>,
}

impl CustomFieldAttribute {
    /// Get whether multiline is enabled.
    pub fn multiline(&self) -> Option<bool> {
        self.multiline
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tooltip {
    #[serde(default)]
    enabled: bool,
    #[serde(default)]
    text: String,
    #[serde(rename = "type")]
    tpe: String,
}

impl Tooltip {
    /// Get whether the tooltip is enabled.
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    /// Get the tooltip text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get the tooltip type.
    pub fn tpe(&self) -> &str {
        &self.tpe
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub enum CustomFieldDataType {
    #[serde(rename = "BOOLEAN")]
    Boolean,
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "ISO_DATE")]
    ISODate,
    #[serde(rename = "SINGLE_SELECT_LIST")]
    SingleSelectList,
}

impl User {
    /// Get the user's unique identifier.
    pub fn uid(&self) -> &str {
        &self.uid
    }

    /// Get the user's email address.
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Get the user's first name.
    pub fn first_name(&self) -> Option<&str> {
        self.first_name.as_deref()
    }

    /// Get the user's last name.
    pub fn last_name(&self) -> Option<&str> {
        self.last_name.as_deref()
    }

    /// Get the user's personal name.
    pub fn personal_name(&self) -> Option<&str> {
        self.personal_name.as_deref()
    }

    /// Get the user's display name.
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    /// Get the user's phone number.
    pub fn phone(&self) -> Option<&str> {
        self.phone.as_deref()
    }

    /// Get the user's creation timestamp.
    pub fn create_date(&self) -> i64 {
        self.create_date
    }

    /// Get the user's last update timestamp.
    pub fn update_date(&self) -> Option<i64> {
        self.update_date
    }

    /// Get the user's status.
    pub fn status(&self) -> Option<&str> {
        self.status.as_deref()
    }

    /// Get the user's custom fields.
    pub fn custom_fields(&self) -> &[CustomField] {
        &self.custom_fields
    }

    /// Get whether a reset password email was sent.
    pub fn reset_password_email_sent(&self) -> bool {
        self.reset_password_email_sent
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PianoPaginated, PianoResponse};
    #[test]
    fn sanity_check_custom_field_decoding() {
        let value = serde_json::json!([
            {
                "archived": false,
                "attribute": {
                  "multiline": false
                },
                "comment": "**MASKED**",
                "created": null,
                "dataType": "TEXT",
                "defaultSortOrder": null,
                "editable": false,
                "emailCreator": null,
                "favouriteOptions": [],
                "fieldDefinitionId": null,
                "fieldName": "**MASKED**",
                "identityId": null,
                "options": [],
                "sortOrder": null,
                "title": "**MASKED**",
                "tooltip": {
                  "enabled": false,
                  "text": "",
                  "type": "InfoIcon"
                },
                "validators": "[]",
                "value": null
            },
            {
                "archived": false,
                "attribute": {},
                "comment": "",
                "created": "2024-11-06T02:55:24.000+00:00",
                "dataType": "BOOLEAN",
                "defaultSortOrder": null,
                "editable": true,
                "emailCreator": "test@example.com",
                "favouriteOptions": null,
                "fieldDefinitionId": null,
                "fieldName": "**MASKED**",
                "identityId": null,
                "options": [],
                "sortOrder": null,
                "title": "**MASKED**",
                "tooltip": {
                    "enabled": false,
                    "text": "",
                    "type": "InfoIcon"
                },
                "validators": "[]",
                "value": "true"
            },
            {
                "archived": false,
                "attribute": {},
                "comment": "",
                "created": "2024-12-23T01:12:24.000+00:00",
                "dataType": "SINGLE_SELECT_LIST",
                "defaultSortOrder": null,
                "editable": false,
                "emailCreator": "test@example.com",
                "favouriteOptions": null,
                "fieldDefinitionId": null,
                "fieldName": "**MASKED**",
                "identityId": null,
                "options": [
                    "**MASKED-A**",
                    "**MASKED-B**",
                    "**MASKED-C**",
                ],
                "sortOrder": null,
                "title": "**MASKED**",
                "tooltip": {
                    "enabled": false,
                    "text": "",
                    "type": "InfoIcon"
                },
                "validators": "[]",
                "value": "[\"**MASKED-A**\"]"
            }
        ]);
        let value = serde_json::from_value::<Vec<CustomField>>(value).expect("OK");
        println!("{value:?}")
    }
    #[test]
    fn sanity_check_custom_field_query_serialization() {
        let data = CustomFieldQuery {
            condition: Some(Condition::Equal {
                options_equal: EqExpr::String {
                    id: 1,
                    value: "foo".to_string(),
                },
            }),
            data_type: Some(CustomFieldDataType::SingleSelectList),
            ..Default::default()
        };
        let str = serde_json::to_string(&data).expect("OK");
        assert_eq!(
            str,
           "{\"condition\":{\"type\":\"EQUAL\",\"optionsEqual\":{\"id\":1,\"value\":\"foo\"}},\"data_type\":\"SINGLE_SELECT_LIST\"}"
        )
    }
    #[test]
    fn test_create_user_request_builder() {
        let request = CreateUserRequest::new("user@example.com")
            .with_first_name("John")
            .with_last_name("Doe")
            .with_personal_name("Johnny");

        assert_eq!(request.email, "user@example.com");
        assert_eq!(request.first_name, Some("John"));
        assert_eq!(request.last_name, Some("Doe"));
        assert_eq!(request.personal_name, Some("Johnny"));
    }

    #[test]
    fn test_list_user_request_builder() {
        let request = ListUserRequest::new()
            .with_limit(50)
            .with_offset(10)
            .with_query("search term");

        assert_eq!(request.limit, Some(50));
        assert_eq!(request.offset, Some(10));
        assert_eq!(request.q, Some("search term"));
    }

    #[test]
    fn test_user_deserialization() {
        let json = serde_json::json!({
            "uid": "12345",
            "email": "user@example.com",
            "first_name": "John",
            "last_name": "Doe",
            "personal_name": "Johnny",
            "create_date": 1640995200,
            "update_date": 1641081600
        });

        let user: User = serde_json::from_value(json).expect("Failed to deserialize user");
        assert_eq!(user.uid(), "12345");
        assert_eq!(user.email(), "user@example.com");
        assert_eq!(user.first_name(), Some("John"));
        assert_eq!(user.last_name(), Some("Doe"));
        assert_eq!(user.personal_name(), Some("Johnny"));
    }

    #[test]
    fn sanity_check_list_users_codec() {
        let snapshot = include_str!("./list.schema.snapshot.json");
        let value = serde_json::from_str::<PianoResponse<PianoPaginated<ListUserResult>>>(snapshot);

        assert!(
            value.is_ok(),
            "Failed to deserialize user list: {:?}",
            value.err()
        );
        let response = value.unwrap();

        match response {
            PianoResponse::Succeed(paginated) => {
                assert_eq!(paginated.limit, 1);
                assert_eq!(paginated.offset, 0);
                assert!(paginated.total >= 165);
                assert!(paginated.count >= 1);

                if !paginated.value.users.is_empty() {
                    let user = &paginated.value.users[0];
                    assert_eq!(user.email(), "test@example.com");
                    assert_eq!(user.uid(), "***MASKED***");
                }
            }
            PianoResponse::Failure { code, message, .. } => {
                panic!("Expected success but got failure: {} - {}", code, message);
            }
        }
    }

    #[test]
    fn sanity_check_get_user_codec() {
        let snapshot = include_str!("./get.schema.snapshot.json");
        let value = serde_json::from_str::<PianoResponse<UserResult>>(snapshot);

        assert!(
            value.is_ok(),
            "Failed to deserialize user get: {:?}",
            value.err()
        );
        let response = value.unwrap();

        match response {
            PianoResponse::Succeed(data) => {
                assert_eq!(data.user.email(), "test@example.com");
                assert_eq!(data.user.uid(), "***MASKED***");
            }
            PianoResponse::Failure { code, message, .. } => {
                panic!("Expected success but got failure: {} - {}", code, message);
            }
        }
    }
}
