const base = "http://query.sse.com.cn/commonQuery.do";
const params = {
  "sqlId": "COMMON_SSE_SJ_GPSJ_GPSJZM_TJSJ_L",
  "PRODUCT_NAME": "股票,主板,科创板",
  "type": "inParams",
};
const headers = {
  "Referer": "http://www.sse.com.cn/",
  "User-Agent":
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.90 Safari/537.36",
};

const url = `${base}?${new URLSearchParams(params)}`;

const response = await fetch(url, { headers });
const json = await response.json();

console.log(json);
