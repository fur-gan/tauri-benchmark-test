import {
  listen,
  emit,
} from "https://unpkg.com/@tauri-apps/api@latest/event.js?module";

class ESize {
  static I = "1kB";
  static X = "10kB";
  static C = "100kB";
}

setTimeout(() => {
  emit(ESize.I, "1000").catch((error) => {
    console.log(`Failed to send message ${ESize.I} to BE: ${error.message}`);
  });
}, 2000);

const maxCount = 1000;
let count = 0;
let time = 0;

function changeTo(index, newStuff) {
  document.getElementById(`${index}`).innerHTML = newStuff;
}

listen(ESize.I, (response) => {
  console.log(count);
  if (count == 0) {
    time = Date.now();
  } else if (count >= maxCount) {
    const duration = Date.now() - time;
    changeTo(
      1,
      `${response.event} Test takes: ${duration} ms (${duration / 1000}s)`
    );
    count = 0;
    emit(ESize.X, "10000").catch((error) => {
      console.log(`Failed to send message ${ESize.X} to BE: ${error.message}`);
    });
    return;
  }
  count++;
});

listen(ESize.X, (response) => {
  if (count == 0) {
    time = Date.now();
  } else if (count >= maxCount) {
    const duration = Date.now() - time;
    changeTo(
      2,
      `${response.event} Test takes: ${duration} ms (${duration / 1000}s)`
    );
    count = 0;
    emit(ESize.C, "100000").catch((error) => {
      console.log(`Failed to send message ${ESize.C} to BE: ${error.message}`);
    });
    return;
  }
  count++;
});

listen(ESize.C, (response) => {
  if (count == 0) {
    time = Date.now();
  } else if (count >= maxCount) {
    const duration = Date.now() - time;
    changeTo(
      3,
      `${response.event} Test takes: ${duration} ms (${duration / 1000}s)`
    );
    count = 0;
    return;
  }
  count++;
});
