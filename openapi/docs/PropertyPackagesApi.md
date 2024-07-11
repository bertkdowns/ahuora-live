# \PropertyPackagesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**property_packages_flowsheet_property_packages_bulk_update_create**](PropertyPackagesApi.md#property_packages_flowsheet_property_packages_bulk_update_create) | **POST** /api/property_packages/flowsheet-property_packages/bulk_update/ | 
[**property_packages_flowsheet_property_packages_get_all_connections_retrieve**](PropertyPackagesApi.md#property_packages_flowsheet_property_packages_get_all_connections_retrieve) | **GET** /api/property_packages/flowsheet-property_packages/get_all_connections/ | 
[**property_packages_flowsheet_property_packages_list**](PropertyPackagesApi.md#property_packages_flowsheet_property_packages_list) | **GET** /api/property_packages/flowsheet-property_packages/ | 
[**property_packages_property_packages_list**](PropertyPackagesApi.md#property_packages_property_packages_list) | **GET** /api/property_packages/property_packages/ | 



## property_packages_flowsheet_property_packages_bulk_update_create

> property_packages_flowsheet_property_packages_bulk_update_create(flowsheet_id, flowsheet_property_package_bulk_update)


Create and delete multiple connections at once.  Parameters: - flowsheetId: The flowsheet ID  Request body: - propertyPackages: List of property package IDs  Returns: - None

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flowsheet_id** | **i32** |  | [required] |
**flowsheet_property_package_bulk_update** | [**FlowsheetPropertyPackageBulkUpdate**](FlowsheetPropertyPackageBulkUpdate.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## property_packages_flowsheet_property_packages_get_all_connections_retrieve

> models::FlowsheetPropertyPackageBulkUpdate property_packages_flowsheet_property_packages_get_all_connections_retrieve(flowsheet_id)


Retrieve the ids of all property packages associated with the given flowsheet ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flowsheet_id** | **i32** |  | [required] |

### Return type

[**models::FlowsheetPropertyPackageBulkUpdate**](FlowsheetPropertyPackageBulkUpdate.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## property_packages_flowsheet_property_packages_list

> Vec<models::FlowsheetPropertyPackage> property_packages_flowsheet_property_packages_list(flowsheet_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flowsheet_id** | **i32** |  | [required] |

### Return type

[**Vec<models::FlowsheetPropertyPackage>**](FlowsheetPropertyPackage.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## property_packages_property_packages_list

> Vec<models::PropertyPackage> property_packages_property_packages_list()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::PropertyPackage>**](PropertyPackage.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

