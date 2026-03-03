<script setup lang="ts">
import { usePlayerStatEditor } from './playerStatEditor';
import { type Player } from '../gameStore';

const props = defineProps<{
    player: Player;
}>();

const playerStatEditor = usePlayerStatEditor(props.player);

const typeBadgeClass = (type: string) => {
    switch (type) {
        case 'number':
            return 'badge badge-secondary badge-sm';
        case 'boolean':
            return 'badge badge-accent badge-sm';
        default:
            return 'badge badge-primary badge-sm';
    }
};
</script>

<template>
    <div class="w-full h-full p-4 overflow-y-auto flex flex-col gap-4">
        <!-- No player -->
        <div v-if="!props.player" class="text-base-content/40 text-sm italic">
            No active player.
        </div>

        <template v-else>
            <!-- Stats list -->
            <div v-if="props.player.stats.length === 0" class="text-base-content/40 text-sm italic">
                This player has no stats yet.
            </div>

            <ul v-else class="flex flex-col gap-2">
                <li
                    v-for="stat in props.player.stats"
                    :key="stat.id"
                    class="bg-base-200 rounded-xl px-4 py-3 flex items-center gap-3"
                >
                    <!-- Key + type badge -->
                    <div class="flex flex-col min-w-0 flex-1">
                        <div class="flex items-center gap-2">
                            <span class="font-medium text-sm truncate">{{ stat.key }}</span>
                            <span :class="typeBadgeClass(stat.valueType)">{{
                                stat.valueType
                            }}</span>
                        </div>

                        <!-- Viewing mode -->
                        <div
                            v-if="playerStatEditor.editingStatId.value !== stat.id"
                            class="flex items-center gap-2 mt-1"
                        >
                            <template v-if="stat.valueType === 'boolean'">
                                <input
                                    type="checkbox"
                                    class="toggle toggle-sm toggle-primary"
                                    :checked="stat.booleanValue ?? false"
                                    @change="
                                        event =>
                                            playerStatEditor.editBooleanStat(
                                                stat,
                                                (event.target as HTMLInputElement).checked
                                            )
                                    "
                                />
                                <span class="text-sm text-base-content/70">
                                    {{ stat.booleanValue ? 'true' : 'false' }}
                                </span>
                            </template>
                            <span v-else class="text-sm text-base-content/70">
                                {{ playerStatEditor.getStatValue(stat) }}
                            </span>
                        </div>

                        <!-- Editing mode -->
                        <div v-else class="flex items-center gap-2 mt-1">
                            <input
                                v-if="stat.valueType === 'number'"
                                v-model="playerStatEditor.editValueRaw.value"
                                type="number"
                                class="input input-sm input-bordered w-32"
                                @keyup.enter="playerStatEditor.saveEdit()"
                                @keyup.escape="playerStatEditor.cancelEditing()"
                                autofocus
                            />
                            <select
                                v-else-if="stat.valueType === 'boolean'"
                                v-model="playerStatEditor.editValueRaw.value"
                                class="select select-sm select-bordered"
                                @keyup.escape="playerStatEditor.cancelEditing()"
                            >
                                <option value="true">true</option>
                                <option value="false">false</option>
                            </select>
                            <input
                                v-else
                                v-model="playerStatEditor.editValueRaw.value"
                                type="text"
                                class="input input-sm input-bordered w-40"
                                @keyup.enter="playerStatEditor.saveEdit()"
                                @keyup.escape="playerStatEditor.cancelEditing()"
                                autofocus
                            />
                            <button
                                class="btn btn-xs btn-primary"
                                @click="playerStatEditor.saveEdit()"
                            >
                                Save
                            </button>
                            <button
                                class="btn btn-xs btn-ghost"
                                @click="playerStatEditor.cancelEditing()"
                            >
                                Cancel
                            </button>
                        </div>
                    </div>

                    <!-- Action buttons (not shown for boolean – edited via toggle) -->
                    <div class="flex gap-1 shrink-0">
                        <button
                            v-if="
                                stat.valueType !== 'boolean' &&
                                playerStatEditor.editingStatId.value !== stat.id
                            "
                            class="btn btn-xs btn-outline"
                            @click="playerStatEditor.startEditing(stat)"
                        >
                            Edit
                        </button>
                        <button
                            class="btn btn-xs btn-error btn-outline"
                            @click="playerStatEditor.removeStat(stat)"
                        >
                            Remove
                        </button>
                    </div>
                </li>
            </ul>

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
