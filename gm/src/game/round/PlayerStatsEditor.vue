<script setup lang="ts">
import { toRef } from 'vue';
import { type Player } from '../gameStore';
import { usePlayerStatEditor } from './playerStatEditor';
import InlineStatEditor from '../stats/InlineStatEditor.vue';

const props = defineProps<{
    player: Player;
}>();

const playerStatEditor = usePlayerStatEditor(toRef(props, 'player'));
</script>

<template>
    <div class="flex flex-col gap-4">
        <!-- No player -->
        <div v-if="!props.player" class="text-base-content/40 text-sm italic">
            No active player.
        </div>

        <template v-else>
            <!-- Stats list -->
            <div v-if="props.player.stats.length === 0" class="text-base-content/40 text-sm italic">
                This player has no stats yet.
            </div>

            <div v-else class="flex flex-col gap-2">
                <InlineStatEditor
                    v-for="stat in props.player.stats"
                    :key="stat.id"
                    :playerId="props.player.id"
                    :stat="stat"
                    size="lg"
                />
            </div>

            <!-- Divider -->
            <div class="divider my-0"></div>

            <!-- Add stat form -->
            <div class="bg-base-200 rounded-xl p-4">
                <h3 class="font-semibold text-sm mb-3">Add Stat</h3>
                <form class="flex flex-col gap-3" @submit.prevent="playerStatEditor.addStat">
                    <div class="flex gap-2 flex-wrap">
                        <input
                            v-model="playerStatEditor.newStatKey.value"
                            type="text"
                            placeholder="Stat name"
                            class="input input-sm input-bordered flex-1 min-w-32"
                        />
                        <select
                            v-model="playerStatEditor.newStatType.value"
                            class="select select-sm select-bordered"
                            @change="playerStatEditor.newStatValue.value = ''"
                        >
                            <option value="string">string</option>
                            <option value="number">number</option>
                            <option value="boolean">boolean</option>
                        </select>
                    </div>

                    <div class="flex gap-2 items-center">
                        <input
                            v-if="playerStatEditor.newStatType.value === 'number'"
                            v-model="playerStatEditor.newStatValue.value"
                            type="number"
                            placeholder="Value"
                            class="input input-sm input-bordered flex-1"
                        />
                        <select
                            v-else-if="playerStatEditor.newStatType.value === 'boolean'"
                            v-model="playerStatEditor.newStatValue.value"
                            class="select select-sm select-bordered flex-1"
                        >
                            <option value="true">true</option>
                            <option value="false">false</option>
                        </select>
                        <input
                            v-else
                            v-model="playerStatEditor.newStatValue.value"
                            type="text"
                            placeholder="Value"
                            class="input input-sm input-bordered flex-1"
                        />
                        <button type="submit" class="btn btn-sm btn-primary">Add</button>
                    </div>

                    <p v-if="playerStatEditor.addStatError" class="text-error text-xs">
                        {{ playerStatEditor.addStatError }}
                    </p>
                </form>
            </div>
        </template>
    </div>
</template>
