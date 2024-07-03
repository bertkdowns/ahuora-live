# \CompoundsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**compounds_compounds_get_valid_property_packages_retrieve**](CompoundsApi.md#compounds_compounds_get_valid_property_packages_retrieve) | **GET** /api/compounds/compounds/{ID}/get_valid_property_packages/ | 
[**compounds_compounds_list**](CompoundsApi.md#compounds_compounds_list) | **GET** /api/compounds/compounds/ | 
[**compounds_compounds_retrieve**](CompoundsApi.md#compounds_compounds_retrieve) | **GET** /api/compounds/compounds/{ID}/ | 
[**compounds_flowsheet_compound_connection_add_connection_create**](CompoundsApi.md#compounds_flowsheet_compound_connection_add_connection_create) | **POST** /api/compounds/flowsheet-compound-connection/add_connection/ | 
[**compounds_flowsheet_compound_connection_get_all_connections_retrieve**](CompoundsApi.md#compounds_flowsheet_compound_connection_get_all_connections_retrieve) | **GET** /api/compounds/flowsheet-compound-connection/{id}/get_all_connections/ | 
[**compounds_flowsheet_compound_connection_remove_connection_create**](CompoundsApi.md#compounds_flowsheet_compound_connection_remove_connection_create) | **POST** /api/compounds/flowsheet-compound-connection/remove_connection/ | 



## compounds_compounds_get_valid_property_packages_retrieve

> std::collections::HashMap<String, serde_json::Value> compounds_compounds_get_valid_property_packages_retrieve(id)


Retrieves all valid property packages for single compound (based on compound ID) a \"valid\" property package is one where an entry exists for that compound type  Parameters: - id: The compound ID  Returns: - list of property packages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique value identifying this compound. | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compounds_compounds_list

> Vec<std::collections::HashMap<String, serde_json::Value>> compounds_compounds_list()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<std::collections::HashMap<String, serde_json::Value>>**](std::collections::HashMap.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compounds_compounds_retrieve

> std::collections::HashMap<String, serde_json::Value> compounds_compounds_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique value identifying this compound. | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compounds_flowsheet_compound_connection_add_connection_create

> models::FlowsheetCompound compounds_flowsheet_compound_connection_add_connection_create(flowsheet_compound)


Creating new link between flowsheet and compound

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flowsheet_compound** | [**FlowsheetCompound**](FlowsheetCompound.md) |  | [required] |

### Return type

[**models::FlowsheetCompound**](FlowsheetCompound.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compounds_flowsheet_compound_connection_get_all_connections_retrieve

> models::FlowsheetCompound compounds_flowsheet_compound_connection_get_all_connections_retrieve(id)


Retrieve all items associated with the given flowsheet ID.  Parameters: - id: The flowsheet ID  Returns: - list of associated connections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this flowsheet compounds. | [required] |

### Return type

[**models::FlowsheetCompound**](FlowsheetCompound.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compounds_flowsheet_compound_connection_remove_connection_create

> models::FlowsheetCompound compounds_flowsheet_compound_connection_remove_connection_create(flowsheet_compound)


Removing link between flowsheet and compound

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flowsheet_compound** | [**FlowsheetCompound**](FlowsheetCompound.md) |  | [required] |

### Return type

[**models::FlowsheetCompound**](FlowsheetCompound.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

