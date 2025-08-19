use newtype_uuid::TypedUuidKind;
use newtype_uuid_macros::impl_typed_uuid_kinds;

impl_typed_uuid_kinds! {
    kinds = {
        HTTPClient = {},
        XMLParser = {},
        APIKey = {},
        IOHandler = {},
        UserAccount = {},
        ProjectTask = {},
    }
}

fn main() {
    // Test that complex names are handled correctly
    let _http_client_tag = HTTPClientKind::tag();
    let _xml_parser_tag = XMLParserKind::tag();
    let _api_key_tag = APIKeyKind::tag();
    let _io_handler_tag = IOHandlerKind::tag();
    let _user_account_tag = UserAccountKind::tag();
    let _project_task_tag = ProjectTaskKind::tag();

    // Test type aliases exist
    let _http_client_uuid: HTTPClientUuid;
    let _xml_parser_uuid: XMLParserUuid;
    let _api_key_uuid: APIKeyUuid;
    let _io_handler_uuid: IOHandlerUuid;
    let _user_account_uuid: UserAccountUuid;
    let _project_task_uuid: ProjectTaskUuid;

    // Test that complex names are properly snake_cased
    assert_eq!(HTTPClientKind::tag().as_str(), "http_client");
    assert_eq!(XMLParserKind::tag().as_str(), "xml_parser");
    assert_eq!(APIKeyKind::tag().as_str(), "api_key");
    assert_eq!(IOHandlerKind::tag().as_str(), "io_handler");
    assert_eq!(UserAccountKind::tag().as_str(), "user_account");
    assert_eq!(ProjectTaskKind::tag().as_str(), "project_task");
}
