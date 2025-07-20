# rkshare_sse

## 数据来源

上海证券交易所

## 数据

### 市场总貌

<https://www.sse.com.cn/market/view/>

#### 分析思路

- 页面中数字具有特殊性，直接在「Network」中搜索
- 得唯一「commonQuery.do」，验其「Response」，诚为矣
  - 注：据其名，猜测所有数据都是该接口

```
https://query.sse.com.cn/commonQuery.do?jsonCallBack=jsonpCallback21205877&isPagination=false&sqlId=COMMON_SSE_SJ_SCGM_C&TRADE_DATE=&_=1752807071918
```

##### 分解

- url: `https://query.sse.com.cn/commonQuery.do`
- query:

  | 键             | 值                      | 义                    |
  | -------------- | ----------------------- | --------------------- |
  | `jsonCallback` | `jsonpCallback21205877` | JSONP 回调，无实意    |
  | `isPagination` | `false`                 | 不分页，其他 API 可能 |
  | `sqlId`        | `COMMON_SSE_SJ_SCGM_C`  | 查询数据库表名        |
  | `TRADE_DATE`   | ``                      | 交易日                |
  | `_`            | `1752807071918`         | 毫秒时间戳            |

##### 嘗試
