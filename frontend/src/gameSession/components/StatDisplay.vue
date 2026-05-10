<script setup lang="ts">
import { computed, ref } from 'vue';
import { useSession } from '../useSession';
import { useCommandEmitter } from '../useCommandEmitter';
import { err, ok, type Result } from 'neverthrow';


const props = defineProps<{
    player: string | null; // Null for game stat, player name for player stat
    statName: string;
    editable: boolean;
}>();

const session = useSession();
const commandEmitter = useCommandEmitter();

const isEditing = ref(false);
const editValue = ref<string>('');

const statTypeKey = computed(() => {
    const statValue = props.player 
        ? session.gameState.value?.player_stats.find(p => p.name === props.statName)?.values.find(v => v[0] === props.player)?.[1] ?? null
        : session.gameState.value?.game_stats.find(s => s.name === props.statName)?.value ?? null;

    if (statValue?.int_value !== null) return 'int_value';
    if (statValue?.float_value !== null) return 'float_value';
    if (statValue?.str_value !== null) return 'str_value';
    if (statValue?.bool_value !== null) return 'bool_value';
    return null;
});

const dataTypeLabel = computed(() => {
    const typeKey = statTypeKey.value;
    if (typeKey === 'int_value') return 'int';
    if (typeKey === 'float_value') return 'float';
    if (typeKey === 'str_value') return 'string';
    if (typeKey === 'bool_value') return 'bool';
    return 'unknown';
});

const value = computed(() => {
    const statValue = props.player 
        ? session.gameState.value?.player_stats.find(p => p.name === props.statName)?.values.find(v => v[0] === props.player)?.[1] ?? null
        : session.gameState.value?.game_stats.find(s => s.name === props.statName)?.value ?? null;

    if (!statValue) return 'N/A';

    if (statValue?.int_value !== null) return statValue.int_value;
    if (statValue?.float_value !== null) return statValue.float_value;
    if (statValue?.str_value !== null) return statValue.str_value;
    if (statValue?.bool_value !== null) return statValue.bool_value;
    return 'N/A';
})
const defaultValue = computed(() => {
    const defaultStatValue = props.player 
        ? session.gameState.value?.player_stats.find(p => p.name === props.statName)?.default ?? null
        : session.gameState.value?.game_stats.find(s => s.name === props.statName)?.default ?? null;

    if (!defaultStatValue) return 'N/A';

    if (defaultStatValue?.int_value !== null) return defaultStatValue.int_value;
    if (defaultStatValue?.float_value !== null) return defaultStatValue.float_value;
    if (defaultStatValue?.str_value !== null) return defaultStatValue.str_value;
    if (defaultStatValue?.bool_value !== null) return defaultStatValue.bool_value;
    return 'N/A';
});

const startEditing = () => {
    editValue.value = String(value.value);
    isEditing.value = true;
};

const convertValue = (newVal: string): Result<string, string> => {
    const typeKey = statTypeKey.value;
    
    if (typeKey === 'int_value') {
        const intVal = parseInt(newVal, 10);
        if (isNaN(intVal)) return err(`Invalid integer: "${newVal}"`);
        return ok("int(" + intVal + ")");
    }
    
    if (typeKey === 'float_value') {
        const floatVal = parseFloat(newVal);
        if (isNaN(floatVal)) return err(`Invalid float: "${newVal}"`);
        return ok('float(' + floatVal + ')');
    }
    
    if (typeKey === 'bool_value') {
        const lower = newVal.toLowerCase();
        if (!['true', 'false', '0', '1', 'yes', 'no'].includes(lower)) {
            return err(`Invalid boolean: "${newVal}". Use true/false, yes/no, or 0/1`);
        }
        return ok("bool(" + ['true', '1', 'yes'].includes(lower) + ")");
    }
    
    return ok("string(" + newVal + ")");
};

const updateValue = () => {
    const newVal = convertValue(editValue.value);

    if (newVal.isErr()) {
        alert(`Error: ${newVal.error}`);
    } else {
        if (props.player) {
            commandEmitter.changePlayerStat(props.player, props.statName, newVal.value);
        } else {
            commandEmitter.changeGameStat(props.statName, newVal.value);
        }
        
        isEditing.value = false;
    }
};

const cancelEditing = () => {
    isEditing.value = false;
};

</script>

<template>
    <div class="flex items-center gap-3 px-3 py-2 bg-base-200 rounded text-sm border">
        <!-- Name -->
        <span class="font-semibold min-w-max">{{ props.statName }}</span>
        
        <!-- Datatype Badge -->
        <span class="badge badge-sm badge-neutral">{{ dataTypeLabel }}</span>
        
        <!-- Value or Input -->
        <div v-if="!isEditing" class="font-semibold">
            {{ value }}
        </div>
        <div v-else class="flex gap-1 items-center w-full">
            <input
                v-if="statTypeKey !== 'bool_value'"
                v-model="editValue"
                type="text"
                class="input input-xs input-bordered flex-1"
                @keydown.enter="updateValue"
                @keydown.escape="cancelEditing"
                autofocus
            />
            <input
                v-else
                type="checkbox"
                :checked="editValue === 'true'"
                class="checkbox checkbox-md"
                @change="event => editValue = (event.target as HTMLInputElement).checked ? 'true' : 'false'"
                @keydown.enter="updateValue"
                @keydown.escape="cancelEditing"
                autofocus
            />
            <button
                class="btn btn-xs btn-success"
                title="Save"
                @click="updateValue"
            >
                <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                </svg>
            </button>
            <button
                class="btn btn-xs btn-error"
                title="Cancel"
                @click="cancelEditing"
            >
                <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                </svg>
            </button>
        </div>
        
        <!-- Edit Button -->
        <button
            v-if="!isEditing && props.editable"
            class="btn btn-xs btn-ghost"
            title="Edit"
            @click="startEditing"
        >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
            </svg>
        </button>
        
        <!-- Default Value (only for game stats) -->
        <div v-if="!props.player" class="ml-auto text-gray-600">
            <span>default:</span>
            <span class="font-semibold ml-1">{{ defaultValue }}</span>
        </div>
    </div>
</template>