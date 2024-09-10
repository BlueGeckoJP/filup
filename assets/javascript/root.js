"use strict";
function onClickUploadButton() {
    let totalSize = 0;
    let totalSizePercent = 0.0;
    const fakeUploadInput = document.getElementById("fake-upload-input");
    const file = fakeUploadInput.files[0];
    let fileName = file.name;
    let fileSize = file.size;
    // URL SAFE BASE64
    const encodedFileName = btoa(fileName)
        .replace(/=/g, "")
        .replace(/\+/g, "-")
        .replace(/\//g, "_");
    const eventSource = new EventSource(`/api/progress?filename=${encodedFileName}`);
    const progressBarText = document.querySelector("#upload-progress-div p");
    const progressBar = document.getElementById("progress-bar-inner");
    eventSource.addEventListener("message", (event) => {
        totalSize += Number(event.data);
        totalSizePercent = Number(((totalSize / fileSize) * 100).toFixed(1));
        progressBarText.innerHTML = `${totalSizePercent}%`;
        progressBar.style.width = `${totalSizePercent}%`;
    });
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
            eventSource.close();
            document.location.reload();
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
function onClickOpenButton(element) {
    const parentId = element.parentNode.parentNode.parentNode
        .id;
    let itemBottomContainer = document.querySelector(`#${parentId} #item-bottom-container`);
    if (itemBottomContainer.style.height !== "auto") {
        const filename = document.querySelector(`#${parentId} .item-filename`).innerHTML;
        let itemFilesize = document.querySelector(`#${parentId} #item-filesize`);
        let itemCreationTime = document.querySelector(`#${parentId} #item-creation-time`);
        if (itemFilesize.innerHTML === "" && itemCreationTime.innerHTML === "") {
            fetch("/api/details", {
                method: "POST",
                body: filename,
            })
                .then((e) => e.json())
                .then((data) => {
                itemFilesize.innerHTML = data.size;
                itemCreationTime.innerHTML = data.creation_time;
                itemBottomContainer.style.height = "auto";
            });
        }
        else {
            itemBottomContainer.style.height = "auto";
        }
    }
    else {
        itemBottomContainer.style.height = "0";
    }
}
function onClickDownloadButton(filepath) {
    let element = document.createElement("a");
    element.href = filepath;
    element.download = "";
    element.click();
}
function onClickRemoveButton(element) {
    if (confirm("Are you sure you want to delete this file?")) {
        const parentId = element.parentNode.parentNode.parentNode
            .id;
        const filename = document.querySelector(`#${parentId} .item-filename`).innerHTML;
        fetch("/api/remove", {
            method: "POST",
            body: filename,
        }).then((e) => {
            if (e.status === 200) {
                alert(`Removed: ${filename}`);
                console.log(`Removed: ${filename}, ${parentId}`);
            }
            else {
                alert("An error occurred during remove file");
            }
            document.location.reload();
        });
    }
}
function onClickHashReloadButton(element) {
    var _a, _b;
    const parentId = element.parentNode.parentNode.parentNode.parentNode.id;
    const filename = document.querySelector(`#${parentId} .item-filename`).innerHTML;
    let sha256HashElement = document.querySelector(`#${parentId} #${((_a = element.parentNode) === null || _a === void 0 ? void 0 : _a.parentNode).id} .item-sha256-hash p`);
    let ripemd160HashElement = document.querySelector(`#${parentId} #${((_b = element.parentNode) === null || _b === void 0 ? void 0 : _b.parentNode).id} .item-ripemd160-hash p`);
    fetch("/api/hash", {
        method: "POST",
        body: filename,
    })
        .then((e) => e.json())
        .then((data) => {
        sha256HashElement.innerHTML = data.sha256;
        ripemd160HashElement.innerHTML = data.ripemd160;
    });
}
function uploadDropHandler(event) {
    event.preventDefault();
    const fakeUploadInput = document.getElementById("fake-upload-input");
    const files = event.dataTransfer.files;
    console.log(`Dropped file: ${files[0].name}`);
    fakeUploadInput.files = files;
    onChangeUploadFile();
}
function uploadDropOverHandler(event) {
    event.preventDefault();
}
document.addEventListener("DOMContentLoaded", (event) => {
    let items = document.querySelectorAll(".item");
    let itemIdCount = 0;
    items.forEach((item) => {
        item.id = `item-${itemIdCount.toString()}`;
        itemIdCount++;
    });
    const fileList = document.getElementById("file-list");
    if (fileList.children.length === 0) {
        let element = document.createElement("p");
        element.innerHTML = "ファイルはありません";
        element.id = "notfound-msg";
        fileList.appendChild(element);
    }
});
