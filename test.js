let promise = Promise.reject(new Error("프라미스 실패!"));
promise.catch(err => console.log('잡았다!'));

// 에러가 잘 처리되었으므로 실행되지 않습니다.
window.addEventListener('unhandledrejection', event => console.log(event.reason));

