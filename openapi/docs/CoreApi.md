# \CoreApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**core_connect_flowsheet_create**](CoreApi.md#core_connect_flowsheet_create) | **POST** /api/core/connect-flowsheet/ | 
[**core_connect_new_stream_create**](CoreApi.md#core_connect_new_stream_create) | **POST** /api/core/connect-new-stream/ | 
[**core_connect_objects_create**](CoreApi.md#core_connect_objects_create) | **POST** /api/core/connect-objects/ | 
[**core_disconnect_flowsheet_create**](CoreApi.md#core_disconnect_flowsheet_create) | **POST** /api/core/disconnect-flowsheet/ | 
[**core_disconnect_objects_create**](CoreApi.md#core_disconnect_objects_create) | **POST** /api/core/disconnect-objects/ | 
[**core_extract_stream_data_create**](CoreApi.md#core_extract_stream_data_create) | **POST** /api/core/extract-stream-data/ | 
[**core_flowsheets_create**](CoreApi.md#core_flowsheets_create) | **POST** /api/core/flowsheets/ | 
[**core_flowsheets_destroy**](CoreApi.md#core_flowsheets_destroy) | **DELETE** /api/core/flowsheets/{id}/ | 
[**core_flowsheets_list**](CoreApi.md#core_flowsheets_list) | **GET** /api/core/flowsheets/ | 
[**core_flowsheets_partial_update**](CoreApi.md#core_flowsheets_partial_update) | **PATCH** /api/core/flowsheets/{id}/ | 
[**core_flowsheets_retrieve**](CoreApi.md#core_flowsheets_retrieve) | **GET** /api/core/flowsheets/{id}/ | 
[**core_flowsheets_update**](CoreApi.md#core_flowsheets_update) | **PUT** /api/core/flowsheets/{id}/ | 
[**core_load_flowsheet_create**](CoreApi.md#core_load_flowsheet_create) | **POST** /api/core/load-flowsheet/ | 
[**core_propertyinfo_create**](CoreApi.md#core_propertyinfo_create) | **POST** /api/core/propertyinfo/ | 
[**core_propertyinfo_destroy**](CoreApi.md#core_propertyinfo_destroy) | **DELETE** /api/core/propertyinfo/{id}/ | 
[**core_propertyinfo_list**](CoreApi.md#core_propertyinfo_list) | **GET** /api/core/propertyinfo/ | 
[**core_propertyinfo_partial_update**](CoreApi.md#core_propertyinfo_partial_update) | **PATCH** /api/core/propertyinfo/{id}/ | 
[**core_propertyinfo_retrieve**](CoreApi.md#core_propertyinfo_retrieve) | **GET** /api/core/propertyinfo/{id}/ | 
[**core_propertyinfo_update**](CoreApi.md#core_propertyinfo_update) | **PUT** /api/core/propertyinfo/{id}/ | 



## core_connect_flowsheet_create

> core_connect_flowsheet_create()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_connect_new_stream_create

> core_connect_new_stream_create(connect_new_material_stream)


Creates a new material stream and connects it to a unit operation @param unitOp: The unit operation to connect to @param portIndex: The port index to connect to on the unit operation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connect_new_material_stream** | [**ConnectNewMaterialStream**](ConnectNewMaterialStream.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_connect_objects_create

> core_connect_objects_create(connect_object)


Connects a material stream and an object :param (json): pk and objectClass of both from and to objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**connect_object** | [**ConnectObject**](ConnectObject.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_disconnect_flowsheet_create

> core_disconnect_flowsheet_create()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_disconnect_objects_create

> core_disconnect_objects_create(disconnect_object)


Disconnects a material stream and an object :param (json): pk and objectClass of both from and to objects

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**disconnect_object** | [**DisconnectObject**](DisconnectObject.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_extract_stream_data_create

> core_extract_stream_data_create()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_flowsheets_create

> models::Flowsheet core_flowsheets_create(flowsheet)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**flowsheet** | Option<[**Flowsheet**](Flowsheet.md)> |  |  |

### Return type

[**models::Flowsheet**](Flowsheet.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_flowsheets_destroy

> core_flowsheets_destroy(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this flowsheet. | [required] |

### Return type

 (empty response body)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_flowsheets_list

> Vec<models::Flowsheet> core_flowsheets_list()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Flowsheet>**](Flowsheet.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_flowsheets_partial_update

> models::Flowsheet core_flowsheets_partial_update(id, patched_flowsheet)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this flowsheet. | [required] |
**patched_flowsheet** | Option<[**PatchedFlowsheet**](PatchedFlowsheet.md)> |  |  |

### Return type

[**models::Flowsheet**](Flowsheet.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_flowsheets_retrieve

> models::Flowsheet core_flowsheets_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this flowsheet. | [required] |

### Return type

[**models::Flowsheet**](Flowsheet.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_flowsheets_update

> models::Flowsheet core_flowsheets_update(id, flowsheet)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this flowsheet. | [required] |
**flowsheet** | Option<[**Flowsheet**](Flowsheet.md)> |  |  |

### Return type

[**models::Flowsheet**](Flowsheet.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_load_flowsheet_create

> core_load_flowsheet_create()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_propertyinfo_create

> models::PropertyInfo core_propertyinfo_create(property_info)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**property_info** | Option<[**PropertyInfo**](PropertyInfo.md)> |  |  |

### Return type

[**models::PropertyInfo**](PropertyInfo.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_propertyinfo_destroy

> core_propertyinfo_destroy(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this property info. | [required] |

### Return type

 (empty response body)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_propertyinfo_list

> Vec<models::PropertyInfo> core_propertyinfo_list()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::PropertyInfo>**](PropertyInfo.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_propertyinfo_partial_update

> models::PropertyInfo core_propertyinfo_partial_update(id, patched_property_info)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this property info. | [required] |
**patched_property_info** | Option<[**PatchedPropertyInfo**](PatchedPropertyInfo.md)> |  |  |

### Return type

[**models::PropertyInfo**](PropertyInfo.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_propertyinfo_retrieve

> models::PropertyInfo core_propertyinfo_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this property info. | [required] |

### Return type

[**models::PropertyInfo**](PropertyInfo.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## core_propertyinfo_update

> models::PropertyInfo core_propertyinfo_update(id, property_info)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this property info. | [required] |
**property_info** | Option<[**PropertyInfo**](PropertyInfo.md)> |  |  |

### Return type

[**models::PropertyInfo**](PropertyInfo.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

