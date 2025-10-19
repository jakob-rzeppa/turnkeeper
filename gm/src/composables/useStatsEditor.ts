import NewStatModal from '@/components/player/NewStatModal.vue'
import { usePlayerEmitter } from '@/emitters/playerEmitter'
import { useModalStore } from '@/stores/modalStore'
import type { PlayerStat } from 'shared-types'
import { ref, watch } from 'vue'

export const useStatsEditor = (props: {
    playerId: number
    playerName: string
    playerStats: PlayerStat[]
}) => {
    const modalStore = useModalStore()
    const playerEmitter = usePlayerEmitter()

    // Map to track changes to stats
    const localStats = ref(new Map<number, string>())
    const isLocalStatsChanged = ref(new Map<number, boolean>())

    watch(
        () => [props.playerId, props.playerStats],
        () => {
            // Initialize local stats and change tracking
            localStats.value.clear()
            isLocalStatsChanged.value.clear()
            props.playerStats.forEach((stat) => {
                localStats.value.set(stat.id, stat.value)
                isLocalStatsChanged.value.set(stat.id, false)
            })
        },
        { immediate: true },
    )

    const handeStatValueChange = (statId: number, newValue: string): void => {
        localStats.value.set(statId, newValue)
        if (props.playerStats.find((s) => s.id === statId)?.value !== newValue) {
            isLocalStatsChanged.value.set(statId, true)
        } else {
            isLocalStatsChanged.value.set(statId, false)
        }
    }

    const openNewStatModal = (): void => {
        modalStore.openModal(NewStatModal, {
            playerId: props.playerId,
            playerName: props.playerName,
        })
    }

    const saveStatChanges = (): void => {
        localStats.value.forEach((value, statId) => {
            if (isLocalStatsChanged.value.get(statId)) {
                playerEmitter.updateStatValueForPlayer(props.playerId, statId, value)
            }
        })
    }

    const removeStatFromPlayer = (statId: number): void => {
        playerEmitter.removeStatFromPlayer(props.playerId, statId)
    }

    return {
        localStats,
        isLocalStatsChanged,
        handeStatValueChange,
        openNewStatModal,
        saveStatChanges,
        removeStatFromPlayer,
    }
}
