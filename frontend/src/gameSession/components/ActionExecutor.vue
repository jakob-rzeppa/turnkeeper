<script setup lang="ts">
import { computed, ref, watch } from 'vue';
import { useCommandEmitter } from '../useCommandEmitter';
import { useSession } from '../useSession';
import { convertValueToStringValue } from '../common/value';

const props = defineProps<{
    actionName: string;
}>();

const session = useSession();
const commandEmitter = useCommandEmitter();

const action = computed(() => session.displayTemplate.value?.actions.find(a => a.name === props.actionName) ?? null);

const params = ref<Record<string, ["int" | "float" | "bool" | "string", string]>>({}); // name -> [datatype, value] (e. g. [int, int(2)])

watch(
    () => action.value,
    newAction => {
        if (newAction) {
            const initialParams: Record<string, ["int" | "float" | "bool" | "string", string]> = {};
            newAction.parameters.forEach(p => {
                const pSplit = p.split(':');
                if (pSplit.length !== 2) {
                    console.warn(`Invalid parameter format for action ${newAction.name}: ${p}`);
                    params.value = {};
                    return;
                }
                const name = pSplit[0];
                const type = pSplit[1];

                if (!name) {
                    console.warn(`Parameter name is empty for action ${newAction.name}`);
                    params.value = {};
                    return;
                }

                if (!type || !['int', 'float', 'string', 'bool'].includes(type)) {
                    console.warn(`Unsupported parameter type for action ${newAction.name}: ${type}`);
                    params.value = {};
                    return;
                }

                const initialValue = type === 'int' || type === 'float' ? '0' : type === 'bool' ? 'false' : '';

                initialParams[name] = [type as "int" | "float" | "bool" | "string", initialValue];
            });
            params.value = initialParams;
        } else {
            params.value = {};
        }
    },
    { immediate: true }
);

const executeAction = () => {
    if (!action.value) return;

    const paramValues: Record<string, string> = {};
    for (const [name, [type, value]] of Object.entries(params.value)) {
        const converted = convertValueToStringValue(type, value);
        if (converted.isErr()) {
            alert(`Error in parameter "${name}": ${converted.error}`);
            return;
        }
        paramValues[name] = converted.value;
    }

    commandEmitter.executeAction(action.value.name, paramValues);
};
</script>

<template>
    <div v-if="action" class="flex items-center gap-3 px-3 py-2 bg-base-200 rounded text-sm border">
        <span class="text-xl font-semibold min-w-max">{{ action.name }}</span>
        <div v-for="([type, value], name) in params" :key="name" class="flex items-center gap-2">
            <span v-if="type === 'bool'" class="label">{{ name }}</span>
            <input 
                v-if="type === 'bool'"
                type="checkbox"
                class="toggle toggle-primary"
                :checked="value === 'true'"
                @change="(e) => params[name]![1] = (e.target as HTMLInputElement).checked ? 'true' : 'false'"
            />
            <label v-else :for="name" class="input input-bordered input-sm flex-1">
                <span class="label">
                    {{ name }}
                    <span class="badge badge-sm badge-secondary">{{ type }}</span>
                </span>
                <input 
                    v-model="params[name]![1]" 
                    :id="name"
                    :type="type === 'int' || type === 'float' ? 'number' : 'text'"
                />
            </label>
        </div>
        <button 
            class="btn btn-primary btn-sm ml-auto"
            @click="executeAction"
        >
            <span>▶</span>
            Execute
        </button>
    </div>
    <p v-else class="text-gray-500 italic">Action not found</p>
</template>