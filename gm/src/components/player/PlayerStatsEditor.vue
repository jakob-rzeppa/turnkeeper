<script setup lang="ts">
import { onUnmounted, ref, watch } from 'vue';
import { usePlayerStore } from '@/stores/playerStore';
import type { Player } from 'shared-types';
import { usePlayerEmitter } from '@/emitters/playerEmitter';
import { useModalStore } from '@/stores/modalStore';
import NewStatModal from './NewStatModal.vue';
import { useAutosaveObject } from '../../composables/useAutosaveObject';

const props = defineProps<{
    playerId: number;
}>();

const playerStore = usePlayerStore();
const modalStore = useModalStore();
const playerEmitter = usePlayerEmitter();

const player = ref<Player | undefined>(undefined);

watch(
    () => playerStore.getPlayerById(props.playerId),
    (newPlayer) => {
        if (newPlayer) {
            player.value = newPlayer;
        }
    },
    { immediate: true, deep: true },
);

const { editableObject, idEditableObjectChanged, saveChanges } = useAutosaveObject<{
    [keyof: string]: {
        name: string;
        type: 'string' | 'number' | 'boolean';
        value: string | boolean;
    };
}>(
    () => {
        const statsRecord: {
            [keyof: string]: {
                name: string;
                type: 'string' | 'number' | 'boolean';
                value: string | boolean;
            };
        } = {};
        player.value?.stats.forEach((stat) => {
            let valueAsStringOrBoolean: string | boolean;

            if (typeof stat.value === 'boolean') {
                valueAsStringOrBoolean = stat.value;
            } else if (typeof stat.value === 'number') {
                valueAsStringOrBoolean = stat.value.toString();
            } else {
                valueAsStringOrBoolean = String(stat.value);
            }

            statsRecord[stat.id.toString()] = {
                name: stat.name,
                type: typeof stat.value as 'string' | 'number' | 'boolean',
                value: valueAsStringOrBoolean,
            };
        });
        return statsRecord;
    },
    (newStats) => {
        Object.keys(newStats).forEach((statId: string) => {
            let value: string | number | boolean;

            switch (newStats[statId].type) {
                case 'number':
                    value = Number(newStats[statId].value);
                    break;
                case 'boolean':
                    if (typeof newStats[statId].value === 'boolean') {
                        value = newStats[statId].value;
                    } else {
                        value = newStats[statId].value === 'false';
                    }
                    break;
                case 'string':
                default:
                    value = String(newStats[statId].value);
            }

            console.log(
                'Saving stat change for statId:',
                statId,
                'with value:',
                value,
                typeof value,
            );

            playerEmitter.updateStatValueForPlayer(
                props.playerId,
                parseInt(statId),
                value,
                newStats[statId].name,
            );
        });
    },
);

onUnmounted(() => {
    saveChanges();
});
</script>

<template>
    <div v-if="!player">Player with Id {{ props.playerId }} not found</div>
    <div v-else class="card bg-base-100 border border-secondary/20">
        <div class="card-body">
            <div class="card-title text-secondary mb-4 flex items-center justify-between">
                <span>Player Stats{{ idEditableObjectChanged ? '*' : '' }}</span>
                <div class="badge badge-secondary badge-outline">
                    {{ player.stats.length }}
                </div>
            </div>

            <div v-if="player.stats.length > 0" class="space-y-3">
                <div
                    v-for="stat in player.stats"
                    :key="stat.id"
                    class="flex items-center p-1 bg-base-200 rounded-xl gap-2 overflow-scroll"
                    @keypress="
                        (e) => {
                            if (e.key === 'Enter') {
                                saveChanges();
                            }
                        }
                    "
                >
                    <div class="flex-1 flex items-center join">
                        <input
                            type="text"
                            class="input input-sm w-auto min-w-[50px] join-item"
                            v-model="editableObject[stat.id].name"
                            :placeholder="`Stat Name`"
                            :size="
                                Math.max(
                                    editableObject[stat.id].name.length * 0.8 || 5,
                                    5,
                                ) /* Math.max ensures the input is at least 5 characters wide */
                            "
                        />
                        <select
                            class="select select-sm w-28 flex-shrink-0 join-item"
                            v-model="editableObject[stat.id].type"
                        >
                            <option value="string">String</option>
                            <option value="number">Number</option>
                            <option
                                value="boolean"
                                @select="() => (editableObject[stat.id].value = '')"
                            >
                                Boolean
                            </option>
                        </select>
                        <input
                            v-if="editableObject[stat.id].type === 'number'"
                            class="input input-sm flex-1 join-item"
                            type="number"
                            v-model="editableObject[stat.id].value"
                        />
                        <input
                            v-if="editableObject[stat.id].type === 'string'"
                            class="input input-sm flex-1 join-item"
                            type="text"
                            v-model="editableObject[stat.id].value"
                        />
                    </div>
                    <input
                        v-if="editableObject[stat.id].type === 'boolean'"
                        class="checkbox checkbox-secondary flex-shrink-0"
                        type="checkbox"
                        v-model="editableObject[stat.id].value"
                    />
                    <button
                        class="btn btn-error btn-sm btn-circle join-item"
                        @click="playerEmitter.removeStatFromPlayer(props.playerId, stat.id)"
                        :title="`Remove ${stat.name}`"
                    >
                        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                            ></path>
                        </svg>
                    </button>
                </div>
            </div>

            <div v-else class="text-center py-6">
                <p class="text-sm text-base-content/60">No stats added yet</p>
            </div>

            <div class="card-actions">
                <button
                    class="btn btn-secondary btn-outline w-full"
                    @click="
                        () =>
                            modalStore.openModal(NewStatModal, {
                                playerId: props.playerId,
                                playerName: player?.name,
                            })
                    "
                >
                    <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M12 6v6m0 0v6m0-6h6m-6 0H6"
                        ></path>
                    </svg>
                    Add New Stat
                </button>
            </div>
        </div>
    </div>
</template>
