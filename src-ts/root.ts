function onClickUploadButton() {
  const fakeUploadInput = document.getElementById(
    "fake-upload-input"
  ) as HTMLInputElement;
  const file = fakeUploadInput.files![0];
  const formData = new FormData();
  formData.append("file", file);
  const action = "/api/upload";
  const options = {
    method: "POST",
    body: formData,
  };
  fetch(action, options).then((e) => {
    if (e.status === 200) {
      alert("Upload complete!");
      document.location.reload();
      return;
    }
    alert("An error occurred during upload");
  });
}

function onChangeUploadFile() {
  const uploadInput = document.getElementById(
    "fake-upload-input"
  ) as HTMLInputElement;
  const uploadFilename = document.getElementById("upload-filename");
  uploadFilename!.innerHTML = uploadInput.files![0].name;
}

function onClickDownloadButton(filepath: string) {
  let element = document.createElement("a");
  element.href = filepath;
  element.download = "";
  element.click();
}

function onClickRemoveButton(element: HTMLElement) {
  if (confirm("Are you sure you want to delete this file?")) {
    const parentId = (element.parentNode!.parentNode as HTMLElement).id;
    const filename = document.querySelector(`#${parentId} p`)!.innerHTML;
    fetch("/api/remove", {
      method: "POST",
      body: filename,
    }).then((e) => {
      if (e.status === 200) {
        alert(`Removed: ${filename}`);
        console.log(`Removed: ${filename}, ${parentId}`);
      } else {
        alert("An error occurred during remove file");
      }
    });
  }

  document.location.reload();
}

function uploadDropHandler(event: DragEvent) {
  event.preventDefault();

  const fakeUploadInput = document.getElementById(
    "fake-upload-input"
  ) as HTMLInputElement;
  const files = event.dataTransfer!.files;

  console.log(`Dropped file: ${files[0].name}`);
  fakeUploadInput.files = files;
  onChangeUploadFile();
}

function uploadDropOverHandler(event: DragEvent) {
  event.preventDefault();
}

document.addEventListener("DOMContentLoaded", (event) => {
  const elements = document.querySelectorAll(".item-uuid");

  elements.forEach((element) => {
    const uuid = self.crypto.randomUUID();
    element.id = `item-${uuid}`;
  });

  const fileList = document.getElementById("file-list") as HTMLElement;
  if (fileList.children.length === 0) {
    let element = document.createElement("p");
    element.innerHTML = "ファイルはありません";
    element.id = "notfound-msg";
    fileList.appendChild(element);
  }

  const eventSource = new EventSource("/api/progress");
  const progressBarText = document.querySelector(
    "#progress-bar p"
  ) as HTMLElement;
  let totalSize = 0;
  eventSource.addEventListener("message", (event) => {
    const fakeUploadInput = document.getElementById(
      "fake-upload-input"
    ) as HTMLInputElement;
    const fileSize = fakeUploadInput.files![0].size;
    totalSize += Number(event.data);
    progressBarText.innerHTML = `${totalSize} / ${fileSize}`;
  });
  eventSource.addEventListener("open", (event) => {
    totalSize = 0;
  });
});
