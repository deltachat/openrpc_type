///! modelled after https://spec.open-rpc.org/
///! The documentation comments are taken from https://github.com/open-rpc/spec/ and licensed under "Apache License 2.0"
use std::collections::HashMap;

use schemars::schema::SchemaObject;
use serde::Deserialize;

// modelled after https://spec.open-rpc.org/#server-variable-object (also takes docs from there)

#[derive(Deserialize, Debug, PartialEq)]
/// A simple object to allow referencing other components in the specification, internally and externally.
/// The Reference Object is defined by JSON Schema and follows the same structure, behavior and rules.
pub struct Reference {
    #[serde(rename = "$ref")]
    r#ref: String,
}
// #[derive(Deserialize, Debug, PartialEq)]
// #[serde(untagged)]
// pub enum OrRef<T> {
//     Value(T),
//     Ref(Reference),
// }

type OrRef<T> = T;

// TODO?
type RuntimeExpression = String;

#[non_exhaustive]
#[derive(Deserialize, Debug, PartialEq)]
pub struct OpenRpcDoc {
    /// REQUIRED. This string MUST be the semantic version number of the OpenRPC Specification version that the OpenRPC document uses.
    /// The openrpc field SHOULD be used by tooling specifications and clients to interpret the OpenRPC document.
    /// This is not related to the API info.version string.
    pub openrpc: String,
    /// REQUIRED. Provides metadata about the API.
    /// The metadata MAY be used by tooling as required.
    pub info: Info,
    /// An array of Server Objects, which provide connectivity information to a target server.
    /// If the servers property is not provided, or is an empty array, the default value would be a Server Object with a url value of localhost.
    #[serde(default)]
    pub servers: Vec<ServerObject>,
    /// REQUIRED. The available methods for the API.
    /// While it is required, the array may be empty (to handle security filtering, for example).
    pub methods: Vec<OrRef<Method>>,
    /// An element to hold various schemas for the specification.
    #[serde(default)]
    pub components: Components,
    /// Additional external documentation.
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct Info {
    /// REQUIRED. The title of the application.
    pub title: String,
    /// A verbose description of the application.
    /// GitHub Flavored Markdown syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// A URL to the Terms of Service for the API.
    /// MUST be in the format of a URL.
    #[serde(rename = "termsOfService")]
    pub terms_of_service: Option<String>,
    /// The contact information for the exposed API.
    pub contact: Option<Contact>,
    /// The license information for the exposed API.
    pub license: Option<License>,
    /// REQUIRED. The version of the OpenRPC document
    /// (which is distinct from the OpenRPC Specification version or the API implementation version).
    pub version: String,
}
#[derive(Deserialize, Debug, PartialEq)]
#[non_exhaustive]
pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
}
#[derive(Deserialize, Debug, PartialEq)]
#[non_exhaustive]
pub struct License {
    /// REQUIRED. The license name used for the API.
    pub name: String,
    /// A URL to the license used for the API. MUST be in the format of a URL.
    pub url: Option<String>,
}
#[derive(Deserialize, Debug, PartialEq)]

pub struct ServerObject {
    /// REQUIRED. A name to be used as the cannonical name for the server.
    pub name: String,
    /// REQUIRED. A URL to the target host.
    /// This URL supports Server Variables and MAY be relative,
    /// to indicate that the host location is relative to the location where the OpenRPC document is being served.
    /// Server Variables are passed into the Runtime Expression to produce a server URL.
    pub url: RuntimeExpression,
    /// A short summary of what the server is.
    pub summary: Option<String>,
    /// An optional string describing the host designated by the URL.
    /// GitHub Flavored Markdown syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// A map between a variable name and its value.
    /// The value is passed into the Runtime Expression to produce a server URL.
    #[serde(default)]
    pub variables: HashMap<String, ServerVariableObject>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ServerVariableObject {
    /// An enumeration of string values to be used if the substitution options are from a limited set.
    #[serde(default)]
    pub r#enum: Vec<String>,
    /// REQUIRED. The default value to use for substitution,
    /// which SHALL be sent if an alternate value is not supplied.
    /// Note this behavior is different than the Schema Object’s treatment of default values,
    /// because in those cases parameter values are optional.
    pub default: String,
    /// An optional description for the server variable.
    /// GitHub Flavored Markdown syntax MAY be used for rich text representation.
    pub description: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
/// Describes the interface for the given method name.
/// The method name is used as the method field of the JSON-RPC body.
/// It therefore MUST be unique.
pub struct Method {
    /// REQUIRED. The cannonical name for the method.
    /// The name MUST be unique within the methods array.
    pub name: String,
    /// A list of tags for API documentation control.
    /// Tags can be used for logical grouping of methods by resources or any other qualifier.
    #[serde(default)]
    pub tags: Vec<OrRef<Tag>>,
    /// A short summary of what the method does.
    pub summary: Option<String>,
    /// A verbose explanation of the method behavior.
    /// GitHub Flavored Markdown syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// Additional external documentation for this method.
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
    /// REQUIRED. A list of parameters that are applicable for this method.
    /// The list MUST NOT include duplicated parameters and therefore require name to be unique.
    /// The list can use the Reference Object to link to parameters that are defined by the Content Descriptor Object.
    /// All optional params (content descriptor objects with “required”: false) MUST be positioned after all required params in the list.
    pub params: Vec<OrRef<ContentDescriptor>>,
    /// The description of the result returned by the method.
    /// If defined, it MUST be a Content Descriptor or Reference Object.
    /// If undefined, the method MUST only be used as a notification.
    pub result: Option<OrRef<ContentDescriptor>>,
    /// Declares this method to be deprecated.
    /// Consumers SHOULD refrain from usage of the declared method. Default value is false.
    #[serde(default = "return_false")]
    pub deprecated: bool,
    /// An alternative servers array to service this method.
    /// If an alternative servers array is specified at the Root level, it will be overridden by this value.
    #[serde(default)]
    pub servers: Vec<ServerObject>,
    /// A list of custom application defined errors that MAY be returned.
    /// The Errors MUST have unique error codes.
    #[serde(default)]
    pub errors: Vec<OrRef<ErrorObject>>,
    /// A list of possible links from this method call.
    #[serde(default)]
    pub links: Vec<OrRef<LinkObject>>,
    /// The expected format of the parameters.
    /// As per the JSON-RPC 2.0 specification, the params of a JSON-RPC request object may be an array, object, or either (represented as by-position, by-name, and either respectively).
    /// When a method has a paramStructure value of by-name, callers of the method MUST send a JSON-RPC request object whose params field is an object.
    /// Further, the key names of the params object MUST be the same as the contentDescriptor.names for the given method. Defaults to "either".
    #[serde(default, rename = "paramStructure")]
    pub param_structure: ParamStructure,
    /// Array of Example Pairing Object where each example includes a valid params-to-result Content Descriptor pairing.
    #[serde(default)]
    pub examples: Vec<ExamplePairingObject>,
}

#[non_exhaustive]
#[derive(Deserialize, Debug, PartialEq)]
pub struct Tag {
    /// REQUIRED. The name of the tag.
    pub name: String,
    /// A short summary of the tag.
    pub summary: Option<String>,
    /// A verbose explanation for the tag.
    /// GitHub Flavored Markdown syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// Additional external documentation for this tag.
    #[serde(rename = "externalDocs")]
    pub external_docs: Option<ExternalDocumentation>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ExternalDocumentation {
    /// A verbose explanation of the target documentation.
    /// GitHub Flavored Markdown syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// REQUIRED. The URL for the target documentation.
    /// Value MUST be in the format of a URL.
    pub url: String,
}

fn return_false() -> bool {
    false
}

#[non_exhaustive]
#[derive(Deserialize, Debug, PartialEq)]
/// Content Descriptors are objects that do just as they suggest - describe content.
/// They are reusable ways of describing either parameters or result. They MUST have a schema.
pub struct ContentDescriptor {
    ///REQUIRED. Name of the content that is being described. If the content described is a method parameter assignable by-name, this field SHALL define the parameter’s key (ie name).
    pub name: String,
    /// A short summary of the content that is being described.
    pub summary: Option<String>,
    ///A verbose explanation of the content descriptor behavior. GitHub Flavored Markdown syntax MAY be used for rich text representation.
    pub description: Option<String>,
    /// Determines if the content is a required field. Default value is false.
    #[serde(default = "return_false")]
    pub required: bool,
    /// REQUIRED. Schema that describes the content.
    pub schema: OrRef<SchemaObject>,
    /// Specifies that the content is deprecated and SHOULD be transitioned out of usage. Default value is false.
    #[serde(default = "return_false")]
    pub deprecated: bool,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ErrorObject {
    /// REQUIRED. A Number that indicates the error type that occurred.
    /// This MUST be an integer.
    /// The error codes from and including -32768 to -32000 are reserved for pre-defined errors.
    /// These pre-defined errors SHOULD be assumed to be returned from any JSON-RPC api.
    pub code: i16,
    /// REQUIRED. A String providing a short description of the error. The message SHOULD be limited to a concise single sentence.
    pub message: String,
    // TODO: data has `any` type, not sure how to best represent this here
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct LinkObject {} // TODO

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum ParamStructure {
    ByName,
    ByPosition,
    Either,
}

impl Default for ParamStructure {
    fn default() -> Self {
        Self::Either
    }
}

#[non_exhaustive]
#[derive(Deserialize, Debug, PartialEq)]
/// The Example Pairing object consists of a set of example params and result.
/// The result is what you can expect from the JSON-RPC service given the exact params.
pub struct ExamplePairingObject {
    /// Name for the example pairing.
    pub name: Option<String>,
    /// A verbose explanation of the example pairing.
    pub description: Option<String>,
    /// Short description for the example pairing.
    pub summary: Option<String>,
    /// Example parameters.
    pub params: Vec<OrRef<ExampleObject>>,
    /// Example result. When undefined, the example pairing represents usage of the method as a notification.
    pub result: Option<OrRef<ExampleObject>>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum ExampleObject {
    Value {
        /// Cannonical name of the example.
        name: Option<String>,
        /// Short description for the example.
        summary: Option<String>,
        /// A verbose explanation of the example.
        /// GitHub Flavored Markdown syntax MAY be used for rich text representation.
        description: Option<String>,
        /// Embedded literal example.
        /// The value field and externalValue field are mutually exclusive.
        /// To represent examples of media types that cannot naturally represented in JSON,
        /// use a string value to contain the example, escaping where necessary.
        value: serde_json::Value,
    },
    ExternalValue {
        /// Cannonical name of the example.
        name: Option<String>,
        /// Short description for the example.
        summary: Option<String>,
        /// A verbose explanation of the example.
        /// GitHub Flavored Markdown syntax MAY be used for rich text representation.
        description: Option<String>,
        /// A URL that points to the literal example.
        /// This provides the capability to reference examples that cannot easily be included in JSON documents.
        /// The value field and externalValue field are mutually exclusive.
        #[serde(rename = "externalValue")]
        external_value: String,
    },
}

#[derive(Deserialize, Debug, Default, PartialEq)]
pub struct Components {
    /// An object to hold reusable Content Descriptor Objects.
    #[serde(
        rename = "contentDescriptors",
        default,
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub content_descriptors: HashMap<String, ContentDescriptor>,
    /// An object to hold reusable Schema Objects.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub schemas: HashMap<String, SchemaObject>,
    /// An object to hold reusable Example Objects.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub examples: HashMap<String, ExampleObject>,
    /// An object to hold reusable Link Objects.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub links: HashMap<String, LinkObject>,
    /// An object to hold reusable Error Objects.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub errors: HashMap<String, ErrorObject>,
    /// An object to hold reusable Example Pairing Objects.
    #[serde(
        rename = "examplePairingObjects",
        default,
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub example_pairing_objects: HashMap<String, ExamplePairingObject>,
    /// An object to hold reusable Tag Objects.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub tags: HashMap<String, Tag>,
}

#[cfg(test)]
mod tests {
    use crate::OpenRpcDoc;

    #[test]
    fn test() {
        let doc: OpenRpcDoc =
            serde_json::from_str(include_str!("../example_files/yerpc_axum_openrpc.json")).unwrap();
        eprintln!("{doc:#?}");
        panic!();
    }
}
