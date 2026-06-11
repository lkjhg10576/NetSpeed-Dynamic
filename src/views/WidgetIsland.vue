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
        <div :class="['status-dot', networkStatus]"></div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { getCurrentWindow, currentMonitor, PhysicalPosition } from '@tauri-apps/api/window';

const uploadSpeed = ref('0 KB/s');
const downloadSpeed = ref('0 KB/s');

// 网络状态指示灯：good(绿), warning(黄), error(红)
const networkStatus = ref<'good' | 'warning' | 'error'>('good');

let lastRx = 0;
let lastTx = 0;
let speedTimer: number;
let pingTimer: number;

const formatSpeed = (bytes: number) => {
    if (bytes < 1024) return bytes + ' B/s';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB/s';
    return (bytes / (1024 * 1024)).toFixed(1) + ' MB/s';
};

// 任务一：纯粹计算流量数字，绝不插手任何状态灯的颜色改变
const fetchSpeedStats = async () => {
    try {
        const [currentRx, currentTx] = await invoke<[number, number]>('get_network_stats');
        if (lastRx !== 0) {
            downloadSpeed.value = formatSpeed(currentRx - lastRx);
            uploadSpeed.value = formatSpeed(currentTx - lastTx);
        }
        lastRx = currentRx;
        lastTx = currentTx;
    } catch (error) {
        console.error('流量获取失败:', error);
    }
};

// 任务二：纯粹通过真实延迟控制状态灯
const checkNetworkLatency = async () => {
    try {
        const latency = await invoke<number>('get_network_latency');

        // 只要能拿到延迟数字，就说明网络肯定是通的（绝不可能是红灯）
        if (latency < 150) {
            networkStatus.value = 'good';      // 延迟优秀，绿色
        } else {
            networkStatus.value = 'warning';   // 延迟高/不稳定，黄色
        }
    } catch (error) {
        // 只有当 Rust 端抛出异常（即 TcpStream 连接超时 1.5 秒或彻底断开）时，才变红
        networkStatus.value = 'error';         // 真正没网，红色
    }
};

const adjustWindowPosition = async () => {
    try {
        const appWindow = getCurrentWindow();
        await new Promise((resolve) => setTimeout(resolve, 150));
        const monitor = await currentMonitor();

        if (monitor) {
            const scaleFactor = await appWindow.scaleFactor();
            const monitorWidthPhysical = monitor.size.width;
            const monitorLeftPhysical = monitor.position.x;
            const monitorTopPhysical = monitor.position.y;

            const windowSize = await appWindow.innerSize();
            const windowWidthPhysical = windowSize.width;

            const x = monitorLeftPhysical + (monitorWidthPhysical - windowWidthPhysical) / 2;
            const y = monitorTopPhysical + (12 * scaleFactor);

            await appWindow.setPosition(new PhysicalPosition(Math.round(x), Math.round(y)));
        }
    } catch (error) {
        console.error('调整窗口位置失败:', error);
    } finally {
        try {
            await getCurrentWindow().show();
        } catch (e) {
            console.error(e);
        }
    }
};

onMounted(async () => {
    await adjustWindowPosition();

    fetchSpeedStats();
    checkNetworkLatency();

    // 流量 1 秒刷一次保证数字灵敏度；延迟 2.5 秒检测一次，避免高频握手本身对带宽造成干扰
    speedTimer = setInterval(fetchSpeedStats, 1000) as unknown as number;
    pingTimer = setInterval(checkNetworkLatency, 2500) as unknown as number;
});

onUnmounted(() => {
    clearInterval(speedTimer);
    clearInterval(pingTimer);
});
</script>

<style scoped>
*,
*::before,
*::after {
    box-sizing: border-box;
    border: none !important;
    outline: none !important;
}

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

.island-container {
    position: absolute;
    top: 0;
    left: 0;
    width: 100% !important;
    height: 100% !important;
    background: rgba(0, 0, 0, 0.95);
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
    transition: background-color 0.4s ease;
}

.good {
    background-color: #34C759;
    /* 绿 */
}

.warning {
    background-color: #FFCC00;
    /* 黄 */
}

.error {
    background-color: #FF3B30;
    /* 红 */
}
</style>