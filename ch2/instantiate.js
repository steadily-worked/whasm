function fetchAndInstantiate(url, importObject) {
  return fetch(url)
    .then((response) => response.arrayBuffer())
    .then((bytes) => WebAssembly.instantiate(bytes, importObject))
    .then((results) => results.instance)
}
// arrayBuffer로 읽은 이후 인스턴스화하는 것 -> 네트워크 전송이 완료된 후에야 컴파일을 함.

;(async (url) => {
  const fetchPromise = fetch(url)
  const { instance } = await WebAssembly.instantiateStreaming(fetchPromise)
  const result = instance.exports.method(param1, param2)
  console.log(result)
})()
// 비동기적으로 처리하여 네트워크에 표시되는 대로 컴파일을 시작할 수 있음
// 이 경우 보통 컴파일 시간이 더 빠르기 때문에, 다운로드 완료 즉시 validation 또한 완료되고 사용할 준비가 됨.
