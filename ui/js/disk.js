if (!window.diskMap) {
    const diskMapJSON = sessionStorage.getItem('diskList');
    if (diskMapJSON) {
        window.diskMap = JSON.parse(diskMapJSON);
    } else {
        window.diskMap = {};
    }
}

async function getDiskInfo() {
    try {
        const disks = await invoke('get_disk_info');

        for (let i = 0; i < disks.length; i++) {
            const disk = disks[i];
            if (disk.mount) {
                window.diskMap[disk.mount] = disk;
            }
        }

        sessionStorage.setItem('diskList', JSON.stringify(window.diskMap));
    } catch (e) {
        alert(`[ERROR] ${e}`);
    }
}

function addDisksToPage() {
    const divDiskList = document.getElementById('disk-list');

    if (!divDiskList) {
        alert('[ERROR] Missing disk-list element!');
        return;
    }

    for (const id in window.diskMap) {
        const html = makeDiskHtml(id);
        if (html) {
            divDiskList.insertAdjacentHTML('beforeend', html);
        }
    }

    // ToDo: add listener
}

function formatBytes(bytes, decimals = 2) {
    if (bytes === 0) return '0.00 Bytes';

    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];

    const i = Math.floor(Math.log(bytes) / Math.log(k));
    const result = (bytes / Math.pow(k, i)).toFixed(dm);

    return result + ' ' + sizes[i];
}

function calculateUsedSpace(availableSpace, totalSize) {
    let usedSpace = totalSize - availableSpace;
    let usedPercentage = (usedSpace / totalSize) * 100;
    return Math.ceil(usedPercentage);
}

function makeDiskHtml(id) {
    const disk = window.diskMap[id];

    if (!disk) {
        return;
    }

    const diskSpaceText = `[${formatBytes(disk.spaceNow)}] of [${formatBytes(disk.spaceMax)}] Free`;

    const usedSpace = calculateUsedSpace(disk.spaceNow, disk.spaceMax);

    return `
<div class="eater-disk">
  <div class="eater-letter-container">
    <h1 id="disk-letter-${id}" class="eater-text">${id}:</h1>
  </div>
  <div class="eater-space-bar">
    <div class="eater-space-bar-text">
      <div class="eater-text-background">
        <span class="eater-text1">${diskSpaceText}</span>
      </div>
    </div>
    <div id="disk-progress-${id}" class="eater-space-bar-value" style="width: ${usedSpace}% !important;"></div>
  </div>
  <div class="eater-check-box">
    <input
      type="checkbox"
      id="disk-check-${id}"
      class="eater-checkbox"
    />
  </div>
</div>
    `;
}
