# \CompoundsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**compounds_compounds_get_valid_property_packages_retrieve**](CompoundsApi.md#compounds_compounds_get_valid_property_packages_retrieve) | **GET** /api/compounds/compounds/{ID}/get_valid_property_packages/ | 
[**compounds_compounds_list**](CompoundsApi.md#compounds_compounds_list) | **GET** /api/compounds/compounds/ | 
[**compounds_compounds_retrieve**](CompoundsApi.md#compounds_compounds_retrieve) | **GET** /api/compounds/compounds/{ID}/ | 
[**compounds_flowsheet_compounds_bulk_update_create**](CompoundsApi.md#compounds_flowsheet_compounds_bulk_update_create) | **POST** /api/compounds/flowsheet-compounds/bulk_update/ | 
[**compounds_flowsheet_compounds_get_all_connections_retrieve**](CompoundsApi.md#compounds_flowsheet_compounds_get_all_connections_retrieve) | **GET** /api/compounds/flowsheet-compounds/get_all_connections/ | 



## compounds_compounds_get_valid_property_packages_retrieve

> models::Compound compounds_compounds_get_valid_property_packages_retrieve(id)


Retrieves all valid property packages for single compound (based on compound ID) a \"valid\" property package is one where an entry exists for that compound type  Parameters: - id: The compound ID  Returns: - list of property packages

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique value identifying this compound. | [required] |

### Return type

[**models::Compound**](Compound.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compounds_compounds_list

> Vec<models::Compound> compounds_compounds_list()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Compound>**](Compound.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compounds_compounds_retrieve

> models::Compound compounds_compounds_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique value identifying this compound. | [required] |

### Return type

[**models::Compound**](Compound.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compounds_flowsheet_compounds_bulk_update_create

> compounds_flowsheet_compounds_bulk_update_create(flowsheet_id, flowsheet_compounds_bulk_update)


Create and delete multiple connections at once.  Parameters: - flowsheetId: The flowsheet ID  Request body: - compounds: List of compound IDs  Returns: - None

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flowsheet_id** | **i32** |  | [required] |
**flowsheet_compounds_bulk_update** | [**FlowsheetCompoundsBulkUpdate**](FlowsheetCompoundsBulkUpdate.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## compounds_flowsheet_compounds_get_all_connections_retrieve

> models::FlowsheetCompoundsBulkUpdate compounds_flowsheet_compounds_get_all_connections_retrieve(flowsheet_id)


Retrieve all items associated with the given flowsheet ID.  Parameters: - id: The flowsheet ID  Returns: - list of associated connections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flowsheet_id** | **i32** |  | [required] |

### Return type

[**models::FlowsheetCompoundsBulkUpdate**](FlowsheetCompoundsBulkUpdate.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

