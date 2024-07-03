# \GraphicsgroupingsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**graphicsgroupings_create**](GraphicsgroupingsApi.md#graphicsgroupings_create) | **POST** /api/graphicsgroupings/ | 
[**graphicsgroupings_destroy**](GraphicsgroupingsApi.md#graphicsgroupings_destroy) | **DELETE** /api/graphicsgroupings/{id}/ | 
[**graphicsgroupings_list**](GraphicsgroupingsApi.md#graphicsgroupings_list) | **GET** /api/graphicsgroupings/ | 
[**graphicsgroupings_partial_update**](GraphicsgroupingsApi.md#graphicsgroupings_partial_update) | **PATCH** /api/graphicsgroupings/{id}/ | 
[**graphicsgroupings_retrieve**](GraphicsgroupingsApi.md#graphicsgroupings_retrieve) | **GET** /api/graphicsgroupings/{id}/ | 
[**graphicsgroupings_update**](GraphicsgroupingsApi.md#graphicsgroupings_update) | **PUT** /api/graphicsgroupings/{id}/ | 



## graphicsgroupings_create

> models::Grouping graphicsgroupings_create(grouping)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grouping** | [**Grouping**](Grouping.md) |  | [required] |

### Return type

[**models::Grouping**](Grouping.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## graphicsgroupings_destroy

> graphicsgroupings_destroy(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this grouping. | [required] |

### Return type

 (empty response body)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## graphicsgroupings_list

> Vec<models::Grouping> graphicsgroupings_list()


### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Grouping>**](Grouping.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## graphicsgroupings_partial_update

> models::Grouping graphicsgroupings_partial_update(id, patched_grouping)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this grouping. | [required] |
**patched_grouping** | Option<[**PatchedGrouping**](PatchedGrouping.md)> |  |  |

### Return type

[**models::Grouping**](Grouping.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## graphicsgroupings_retrieve

> models::Grouping graphicsgroupings_retrieve(id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this grouping. | [required] |

### Return type

[**models::Grouping**](Grouping.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## graphicsgroupings_update

> models::Grouping graphicsgroupings_update(id, grouping)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | A unique integer value identifying this grouping. | [required] |
**grouping** | [**Grouping**](Grouping.md) |  | [required] |

### Return type

[**models::Grouping**](Grouping.md)

### Authorization

[jwtAuth](../README.md#jwtAuth)

### HTTP request headers

- **Content-Type**: application/json, application/x-www-form-urlencoded, multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

