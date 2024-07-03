# \PropertyPackagesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**property_packages_flowsheet_property_packages_connection_add_connection_create**](PropertyPackagesApi.md#property_packages_flowsheet_property_packages_connection_add_connection_create) | **POST** /api/property_packages/flowsheet-property_packages-connection/add_connection/ | 
[**property_packages_flowsheet_property_packages_connection_get_all_connections_retrieve**](PropertyPackagesApi.md#property_packages_flowsheet_property_packages_connection_get_all_connections_retrieve) | **GET** /api/property_packages/flowsheet-property_packages-connection/{id}/get_all_connections/ | 
[**property_packages_flowsheet_property_packages_connection_remove_connection_create**](PropertyPackagesApi.md#property_packages_flowsheet_property_packages_connection_remove_connection_create) | **POST** /api/property_packages/flowsheet-property_packages-connection/remove_connection/ | 
[**property_packages_property_packages_list**](PropertyPackagesApi.md#property_packages_property_packages_list) | **GET** /api/property_packages/property_packages/ | 



## property_packages_flowsheet_property_packages_connection_add_connection_create

> models::FlowsheetPropertyPackage property_packages_flowsheet_property_packages_connection_add_connection_create(flowsheet_property_package)


Creating new link between flowsheet and compound

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flowsheet_property_package** | [**FlowsheetPropertyPackage**](FlowsheetPropertyPackage.md) |  | [required] |

### Return type

[**models::FlowsheetPropertyPackage**](FlowsheetPropertyPackage.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## property_packages_flowsheet_property_packages_connection_get_all_connections_retrieve

> models::FlowsheetPropertyPackage property_packages_flowsheet_property_packages_connection_get_all_connections_retrieve(id)


Retrieve all items associated with the given flowsheet ID.  Parameters: - id: The flowsheet ID  Returns: - list of associated connections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this flowsheet compounds. | [required] |

### Return type

[**models::FlowsheetPropertyPackage**](FlowsheetPropertyPackage.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## property_packages_flowsheet_property_packages_connection_remove_connection_create

> models::FlowsheetPropertyPackage property_packages_flowsheet_property_packages_connection_remove_connection_create(flowsheet_property_package)


Removing link between flowsheet and property package

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flowsheet_property_package** | [**FlowsheetPropertyPackage**](FlowsheetPropertyPackage.md) |  | [required] |

### Return type

[**models::FlowsheetPropertyPackage**](FlowsheetPropertyPackage.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
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

