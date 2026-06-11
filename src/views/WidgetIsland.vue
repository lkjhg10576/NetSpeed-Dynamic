<template>
    <div class="island-container" data-tauri-drag-region>
        <div class="speed-box" data-tauri-drag-region>
            <div class="speed-item">
                <span class="label">↑</span>
                <span class="value">{{ uploadSpeed }}</span>
            </div>
            <div class="divider"></div>
            <div class="speed-item">
                <span class="label">↓</span>
                <span class="value">{{ downloadSpeed }}</span>
            </div>
        </div>
        <div :class="['status-dot', isLagging ? 'lag' : 'good']"></div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow, currentMonitor, PhysicalPosition } from '@tauri-apps/api/window';

const uploadSpeed = ref('0 KB/s');
const downloadSpeed = ref('0 KB/s');
const isLagging = ref(false);

let lastRx = 0;
let lastTx = 0;
let timer: number;

const formatSpeed = (bytes: number) => {
    if (bytes < 1024) return bytes + ' B/s';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB/s';
    return (bytes / (1024 * 1024)).toFixed(1) + ' MB/s';
};

const fetchStats = async () => {
    try {
        const [currentRx, currentTx] = await invoke<[number, number]>('get_network_stats');
        if (lastRx !== 0) {
            const rxDiff = currentRx - lastRx;
            const txDiff = currentTx - lastTx;
            downloadSpeed.value = formatSpeed(rxDiff);
            uploadSpeed.value = formatSpeed(txDiff);
            isLagging.value = rxDiff < 100 && txDiff < 100 ? Math.random() > 0.95 : false;
        }
        lastRx = currentRx;
        lastTx = currentTx;
    } catch (error) {
        console.error('网速获取失败:', error);
    }
};

// 完美适配全分辨率、全缩放率（如 150%）的无缝居中算法
const adjustWindowPosition = async () => {
    try {
        const appWindow = getCurrentWindow();

        // 等待 Windows DWM 与 WebView2 完成 DPI 握手
        await new Promise((resolve) => setTimeout(resolve, 150));

        const monitor = await currentMonitor();

        if (monitor) {
            const scaleFactor = await appWindow.scaleFactor();

            // 获取当前屏幕的【物理像素】总宽度和起跑线 X/Y 坐标
            const monitorWidthPhysical = monitor.size.width;
            const monitorLeftPhysical = monitor.position.x;
            const monitorTopPhysical = monitor.position.y;

            // 直接向系统索要当前窗口的真实【物理尺寸】
            const windowSize = await appWindow.innerSize();
            const windowWidthPhysical = windowSize.width;

            // 在纯物理像素世界里进行绝对居中计算
            const x = monitorLeftPhysical + (monitorWidthPhysical - windowWidthPhysical) / 2;

            // 距离顶部留出 12px 的逻辑高度，同样转换为物理高度
            const y = monitorTopPhysical + (12 * scaleFactor);

            // 使用 PhysicalPosition 物理坐标定死位置
            await appWindow.setPosition(new PhysicalPosition(Math.round(x), Math.round(y)));
        }
    } catch (error) {
        console.error('调整窗口位置失败:', error);
    } finally {
        // 这里必须是 finally，确保无论定位成功或失败，最后都一定会把窗口秀出来
        try {
            await getCurrentWindow().show();
        } catch (e) {
            console.error(e);
        }
    }
};

onMounted(async () => {
    await adjustWindowPosition();
    fetchStats();
    timer = setInterval(fetchStats, 1000) as unknown as number;
});

onUnmounted(() => {
    clearInterval(timer);
});
</script>

<style scoped>
/* 斩断一切 webview 默认边框和溢出 */
*,
*::before,
*::after {
    box-sizing: border-box;
    border: none !important;
    outline: none !important;
}

/* 添加：强制隐藏可能泄露的系统元素 */
:root {
    -webkit-app-region: drag;
}


:global(html),
:global(body) {
    background-color: transparent !important;
    background: transparent !important;
    overflow: hidden;
    margin: 0;
    padding: 0;
    border: none !important;
}

/* 灵动岛核心样式 - 干净利落的果味胶囊 */
.island-container {
    position: absolute;
    top: 0;
    left: 0;
    /* 确保完全覆盖窗口 */
    width: 100% !important;
    height: 100% !important;
    background: rgba(0, 0, 0, 0.95);
    /* 调深色调，全面隐匿可能存在的边缘缝隙 */
    backdrop-filter: blur(20px);
    border-radius: 18px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 14px;
    color: white;
    user-select: none;
    box-shadow: none !important;
    border: none !important;
}

[data-tauri-drag-region] {
    -webkit-app-region: drag;
    cursor: grab;
}

[data-tauri-drag-region]:active {
    cursor: grabbing;
}

.speed-box {
    display: flex;
    align-items: center;
    gap: 10px;
}

.speed-item {
    display: flex;
    align-items: center;
    gap: 4px;
    font-family: -apple-system, BlinkMacSystemFont, 'SF Pro Text', sans-serif;
}

.label {
    font-size: 10px;
    color: rgba(255, 255, 255, 0.4);
    font-weight: bold;
}

.value {
    font-size: 11px;
    font-weight: 500;
    min-width: 52px;
    letter-spacing: -0.2px;
}

.divider {
    width: 1px;
    height: 12px;
    background: rgba(255, 255, 255, 0.12);
}

.status-dot {
    width: 6px;
    height: 6px;
    border-radius: 50%;
}

.good {
    background-color: #34C759;
}

.lag {
    background-color: #FF3B30;
}
</style>