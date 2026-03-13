import { ref, type Ref } from 'vue';
import { useEventEmitter } from '../../events/useEventEmitter';
import { type Player, type Stat } from '../gameStore';

/**
 * @param player The index of the player whose stats are being edited.
 */
export const usePlayerStatEditor = (player: Ref<Player>) => {
    const eventEmitter = useEventEmitter();

    const getStatValue = (stat: Stat): number | string | boolean => {
        if (stat.valueType === 'number') return stat.numberValue ?? 0;
        if (stat.valueType === 'string') return stat.stringValue ?? '';
        return stat.booleanValue ?? false;
    };

    // --- Inline editing ---
    const editingStatId = ref<string | null>(null);
    const editValueRaw = ref('');

    const startEditing = (stat: Stat) => {
        editingStatId.value = stat.id;
        editValueRaw.value = String(getStatValue(stat));
    };

    const cancelEditing = () => {
        editingStatId.value = null;
        editValueRaw.value = '';
    };

    const saveEdit = () => {
        const stat = player.value?.stats.find(s => s.id === editingStatId.value);
        if (!stat) {
            cancelEditing();
            return;
        }

        if (!player.value) return;

        let parsedValue: number | string | boolean = editValueRaw.value;
        if (stat.valueType === 'number') parsedValue = parseFloat(editValueRaw.value) || 0;
        if (stat.valueType === 'boolean') parsedValue = editValueRaw.value === 'true';

        eventEmitter.changeStatOfPlayer(
            player.value.id,
            stat.id,
            stat.valueType,
            parsedValue.toString()
        );
        cancelEditing();
    };

    const editBooleanStat = (stat: Stat, newValue: boolean) => {
        if (stat.valueType !== 'boolean') return;
        if (!player.value) return;

        eventEmitter.changeStatOfPlayer(player.value.id, stat.id, 'boolean', newValue.toString());
    };

    // --- Remove ---
    const removeStat = (stat: Stat) => {
        if (!player.value) return;
        eventEmitter.removeStatFromPlayer(player.value.id, stat.id);
    };

    // --- Add stat ---
    const newStatKey = ref('');
    const newStatType = ref<'string' | 'number' | 'boolean'>('string');
    const newStatValue = ref('');
    const addStatError = ref('');

    const addStat = () => {
        addStatError.value = '';
        if (!player.value) return;

        const trimmedKey = newStatKey.value.trim();
        if (!trimmedKey) {
            addStatError.value = 'Stat name is required.';
            return;
        }

        const duplicate = player.value.stats.some(s => s.key === trimmedKey);
        if (duplicate) {
            addStatError.value = `A stat named "${trimmedKey}" already exists.`;
            return;
        }

        let value: number | string | boolean = newStatValue.value;
        if (newStatType.value === 'number') value = parseFloat(newStatValue.value) || 0;
        if (newStatType.value === 'boolean') value = newStatValue.value === 'true';

        eventEmitter.addStatToPlayer(
            player.value.id,
            trimmedKey,
            newStatType.value,
            value.toString()
        );

        newStatKey.value = '';
        newStatValue.value = '';
    };

    return {
        getStatValue,
        editingStatId,
        editValueRaw,
        editBooleanStat,
        startEditing,
        cancelEditing,
        saveEdit,
        removeStat,
        newStatKey,
        newStatType,
        newStatValue,
        addStatError,
        addStat,
    };
};
