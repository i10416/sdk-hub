//! HubSpot Object Types
//!
//! This module contains type definitions for HubSpot object type IDs.
//! Each type implements AsRef<&str> to provide the corresponding object type ID.
//!
//! See also:
//! - https://knowledge.hubspot.com/integrations/connect-hubspot-and-aws-s3

use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Display};

/// Information about individuals interacting with your business
#[derive(Debug)]
pub struct ContactTypeId;

impl ContactTypeId {
    pub fn value() -> &'static str {
        "0-1"
    }
}

impl AsRef<str> for ContactTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for ContactTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for ContactTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for ContactTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(ContactTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid ContactTypeId: expected '0-1', got '{}'",
                s
            )))
        }
    }
}

/// Information about individual businesses or organizations
#[derive(Debug)]
pub struct CompanyTypeId;

impl CompanyTypeId {
    pub fn value() -> &'static str {
        "0-2"
    }
}

impl AsRef<str> for CompanyTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for CompanyTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for CompanyTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for CompanyTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(CompanyTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid CompanyTypeId: expected '0-2', got '{}'",
                s
            )))
        }
    }
}

/// Details about revenue opportunities with a contact or company
#[derive(Debug)]
pub struct DealTypeId;

impl DealTypeId {
    pub fn value() -> &'static str {
        "0-3"
    }
}

impl AsRef<str> for DealTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for DealTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for DealTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for DealTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(DealTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid DealTypeId: expected '0-3', got '{}'",
                s
            )))
        }
    }
}

/// Stores data from CRM actions, including notes, tasks, emails, meetings, and calls
#[derive(Debug)]
pub struct EngagementTypeId;

impl EngagementTypeId {
    pub fn value() -> &'static str {
        "0-4"
    }
}

impl AsRef<str> for EngagementTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for EngagementTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for EngagementTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for EngagementTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(EngagementTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid EngagementTypeId: expected '0-4', got '{}'",
                s
            )))
        }
    }
}

/// Represent customer requests for help or support
#[derive(Debug)]
pub struct TicketTypeId;

impl TicketTypeId {
    pub fn value() -> &'static str {
        "0-5"
    }
}

impl AsRef<str> for TicketTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for TicketTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for TicketTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for TicketTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(TicketTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid TicketTypeId: expected '0-5', got '{}'",
                s
            )))
        }
    }
}

/// Used to share pricing information with potential buyers
#[derive(Debug)]
pub struct QuoteTypeId;

impl QuoteTypeId {
    pub fn value() -> &'static str {
        "0-14"
    }
}

impl AsRef<str> for QuoteTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for QuoteTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for QuoteTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for QuoteTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(QuoteTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid QuoteTypeId: expected '0-14', got '{}'",
                s
            )))
        }
    }
}

/// Details for individual submissions for a HubSpot form
#[derive(Debug)]
pub struct FormSubmissionTypeId;

impl FormSubmissionTypeId {
    pub fn value() -> &'static str {
        "0-15"
    }
}

impl AsRef<str> for FormSubmissionTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for FormSubmissionTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for FormSubmissionTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for FormSubmissionTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(FormSubmissionTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid FormSubmissionTypeId: expected '0-15', got '{}'",
                s
            )))
        }
    }
}

/// Represent a subset of products sold in a deal. When a product is attached to a deal, it becomes a line item
#[derive(Debug)]
pub struct LineItemTypeId;

impl LineItemTypeId {
    pub fn value() -> &'static str {
        "0-8"
    }
}

impl AsRef<str> for LineItemTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for LineItemTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for LineItemTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for LineItemTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(LineItemTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid LineItemTypeId: expected '0-8', got '{}'",
                s
            )))
        }
    }
}

/// Details of incoming messages from multiple channels
#[derive(Debug)]
pub struct ConversationTypeId;

impl ConversationTypeId {
    pub fn value() -> &'static str {
        "0-11"
    }
}

impl AsRef<str> for ConversationTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for ConversationTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for ConversationTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for ConversationTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(ConversationTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid ConversationTypeId: expected '0-11', got '{}'",
                s
            )))
        }
    }
}

/// Details of your landing pages
#[derive(Debug)]
pub struct LandingPageTypeId;

impl LandingPageTypeId {
    pub fn value() -> &'static str {
        "0-25"
    }
}

impl AsRef<str> for LandingPageTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for LandingPageTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for LandingPageTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for LandingPageTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(LandingPageTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid LandingPageTypeId: expected '0-25', got '{}'",
                s
            )))
        }
    }
}

/// Stores information about to-do lists
#[derive(Debug)]
pub struct TaskTypeId;

impl TaskTypeId {
    pub fn value() -> &'static str {
        "0-27"
    }
}

impl AsRef<str> for TaskTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for TaskTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for TaskTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for TaskTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(TaskTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid TaskTypeId: expected '0-27', got '{}'",
                s
            )))
        }
    }
}

/// Used to collect lead information about your visitors and contacts
#[derive(Debug)]
pub struct FormTypeId;

impl FormTypeId {
    pub fn value() -> &'static str {
        "0-28"
    }
}

impl AsRef<str> for FormTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for FormTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for FormTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for FormTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(FormTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid FormTypeId: expected '0-28', got '{}'",
                s
            )))
        }
    }
}

/// Details about emails from marketing hub
#[derive(Debug)]
pub struct MarketingEmailTypeId;

impl MarketingEmailTypeId {
    pub fn value() -> &'static str {
        "0-29"
    }
}

impl AsRef<str> for MarketingEmailTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for MarketingEmailTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for MarketingEmailTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for MarketingEmailTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(MarketingEmailTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid MarketingEmailTypeId: expected '0-29', got '{}'",
                s
            )))
        }
    }
}

/// Stores information about your ad accounts on Linkedin, Facebook and Google
#[derive(Debug)]
pub struct AdAccountTypeId;

impl AdAccountTypeId {
    pub fn value() -> &'static str {
        "0-30"
    }
}

impl AsRef<str> for AdAccountTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for AdAccountTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for AdAccountTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for AdAccountTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(AdAccountTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid AdAccountTypeId: expected '0-30', got '{}'",
                s
            )))
        }
    }
}

/// Details about your ad campaigns. An ad campaign can contain one or more ads
#[derive(Debug)]
pub struct AdCampaignTypeId;

impl AdCampaignTypeId {
    pub fn value() -> &'static str {
        "0-31"
    }
}

impl AsRef<str> for AdCampaignTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for AdCampaignTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for AdCampaignTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for AdCampaignTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(AdCampaignTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid AdCampaignTypeId: expected '0-31', got '{}'",
                s
            )))
        }
    }
}

/// Logical grouping of ads within an ad campaign
#[derive(Debug)]
pub struct AdGroupTypeId;

impl AdGroupTypeId {
    pub fn value() -> &'static str {
        "0-32"
    }
}

impl AsRef<str> for AdGroupTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for AdGroupTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for AdGroupTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for AdGroupTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(AdGroupTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid AdGroupTypeId: expected '0-32', got '{}'",
                s
            )))
        }
    }
}

/// Details about individual ads
#[derive(Debug)]
pub struct AdTypeId;

impl AdTypeId {
    pub fn value() -> &'static str {
        "0-33"
    }
}

impl AsRef<str> for AdTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for AdTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for AdTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for AdTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(AdTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid AdTypeId: expected '0-33', got '{}'",
                s
            )))
        }
    }
}

/// Information about related marketing assets and content, so you can easily measure the effectiveness of your collective marketing efforts
#[derive(Debug)]
pub struct CampaignTypeId;

impl CampaignTypeId {
    pub fn value() -> &'static str {
        "0-35"
    }
}

impl AsRef<str> for CampaignTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for CampaignTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for CampaignTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for CampaignTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(CampaignTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid CampaignTypeId: expected '0-35', got '{}'",
                s
            )))
        }
    }
}

/// Data about individual pages on your websites
#[derive(Debug)]
pub struct SitePageTypeId;

impl SitePageTypeId {
    pub fn value() -> &'static str {
        "0-38"
    }
}

impl AsRef<str> for SitePageTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for SitePageTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for SitePageTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for SitePageTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(SitePageTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid SitePageTypeId: expected '0-38', got '{}'",
                s
            )))
        }
    }
}

/// Data about blog posts
#[derive(Debug)]
pub struct BlogPostTypeId;

impl BlogPostTypeId {
    pub fn value() -> &'static str {
        "0-39"
    }
}

impl AsRef<str> for BlogPostTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for BlogPostTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for BlogPostTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for BlogPostTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(BlogPostTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid BlogPostTypeId: expected '0-39', got '{}'",
                s
            )))
        }
    }
}

/// Information about groupings of object records based on their properties or activities
#[derive(Debug)]
pub struct ObjectListTypeId;

impl ObjectListTypeId {
    pub fn value() -> &'static str {
        "0-45"
    }
}

impl AsRef<str> for ObjectListTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for ObjectListTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for ObjectListTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for ObjectListTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(ObjectListTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid ObjectListTypeId: expected '0-45', got '{}'",
                s
            )))
        }
    }
}

/// Calls made by other CRM records, for example contacts
#[derive(Debug)]
pub struct CallTypeId;
impl CallTypeId {
    pub fn value() -> &'static str {
        "0-48"
    }
}

impl AsRef<str> for CallTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for CallTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for CallTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for CallTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(CallTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid CallTypeId: expected '0-48', got '{}'",
                s
            )))
        }
    }
}

/// Manage and sync invoices with external accounting systems
#[derive(Debug)]
pub struct InvoiceTypeId;
impl InvoiceTypeId {
    pub fn value() -> &'static str {
        "0-53"
    }
}

impl AsRef<str> for InvoiceTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for InvoiceTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for InvoiceTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for InvoiceTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(InvoiceTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid InvoiceTypeId: expected '0-53', got '{}'",
                s
            )))
        }
    }
}

/// Information about media assets imported into HubSpot
#[derive(Debug)]
pub struct MediaBridgeTypeId;

impl MediaBridgeTypeId {
    pub fn value() -> &'static str {
        "0-57"
    }
}

impl AsRef<str> for MediaBridgeTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for MediaBridgeTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for MediaBridgeTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for MediaBridgeTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(MediaBridgeTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid MediaBridgeTypeId: expected '0-57', got '{}'",
                s
            )))
        }
    }
}

/// A sequence is a series of targeted, timed email templates to nurture contacts over time
#[derive(Debug)]
pub struct SequenceTypeId;

impl SequenceTypeId {
    pub fn value() -> &'static str {
        "0-58"
    }
}

impl AsRef<str> for SequenceTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for SequenceTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for SequenceTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for SequenceTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(SequenceTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid SequenceTypeId: expected '0-58', got '{}'",
                s
            )))
        }
    }
}

/// Deal splits are used to share deal credit among multiple users
#[derive(Debug)]
pub struct DealSplitTypeId;

impl DealSplitTypeId {
    pub fn value() -> &'static str {
        "0-72"
    }
}

impl AsRef<str> for DealSplitTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for DealSplitTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for DealSplitTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for DealSplitTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(DealSplitTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid DealSplitTypeId: expected '0-72', got '{}'",
                s
            )))
        }
    }
}

/// Sales documents build a library of content for your entire team to upload and share documents with your contacts
#[derive(Debug)]
pub struct SalesDocumentTypeId;

impl SalesDocumentTypeId {
    pub fn value() -> &'static str {
        "0-83"
    }
}

impl AsRef<str> for SalesDocumentTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for SalesDocumentTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for SalesDocumentTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for SalesDocumentTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(SalesDocumentTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid SalesDocumentTypeId: expected '0-83', got '{}'",
                s
            )))
        }
    }
}

/// Stores information submitted to a feedback survey
#[derive(Debug)]
pub struct FeedbackSubmissionTypeId;

impl FeedbackSubmissionTypeId {
    pub fn value() -> &'static str {
        "0-19"
    }
}

impl AsRef<str> for FeedbackSubmissionTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for FeedbackSubmissionTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for FeedbackSubmissionTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for FeedbackSubmissionTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(FeedbackSubmissionTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid FeedbackSubmissionTypeId: expected '0-19', got '{}'",
                s
            )))
        }
    }
}

/// Subscriptions contain details of recurring payments
#[derive(Debug)]
pub struct SubscriptionTypeId;

impl SubscriptionTypeId {
    pub fn value() -> &'static str {
        "0-69"
    }
}

impl AsRef<str> for SubscriptionTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for SubscriptionTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for SubscriptionTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for SubscriptionTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(SubscriptionTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid SubscriptionTypeId: expected '0-69', got '{}'",
                s
            )))
        }
    }
}

/// Contains data about funds collected from customers
#[derive(Debug)]
pub struct CommercePaymentTypeId;
impl CommercePaymentTypeId {
    pub fn value() -> &'static str {
        "0-101"
    }
}

impl AsRef<str> for CommercePaymentTypeId {
    fn as_ref(&self) -> &str {
        Self::value()
    }
}

impl Display for CommercePaymentTypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}

impl Serialize for CommercePaymentTypeId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(Self::value())
    }
}

impl<'de> Deserialize<'de> for CommercePaymentTypeId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == Self::value() {
            Ok(CommercePaymentTypeId)
        } else {
            Err(D::Error::custom(format!(
                "Invalid CommercePaymentTypeId: expected '0-101', got '{}'",
                s
            )))
        }
    }
}
