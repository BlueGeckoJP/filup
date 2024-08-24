function onClickUploadButton() {
  const file = document.getElementById("fake-upload-input").files[0];
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
      return;
    }
    alert("An error occurred during upload");
  });
}

function onChangeUploadFile() {
  const uploadInput = document.getElementById("fake-upload-input");
  const uploadFilename = document.getElementById("upload-filename");
  uploadFilename.innerHTML = uploadInput.files[0].name;
}
