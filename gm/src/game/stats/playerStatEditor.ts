import { ref } from 'vue';
import { useEventEmitter } from '../../events/useEventEmitter';
import { type Player, type Stat } from '../gameStore';

/**
 * @param player The index of the player whose stats are being edited.
 */
export const usePlayerStatEditor = (player: Player) => {
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
        const stat = player?.stats.find(s => s.id === editingStatId.value);
        if (!stat) {
            cancelEditing();
            return;
        }

        if (!player) return;

        let parsedValue: number | string | boolean = editValueRaw.value;
        if (stat.valueType === 'number') parsedValue = parseFloat(editValueRaw.value) || 0;
        if (stat.valueType === 'boolean') parsedValue = editValueRaw.value === 'true';

        eventEmitter.emit({
            ChangeStatOfPlayer: {
                player_id: player.id,
                stat_id: stat.id,
                stat_type: stat.valueType,
                stat_value: parsedValue.toString(),
            },
        });
        cancelEditing();
    };

    const editBooleanStat = (stat: Stat, newValue: boolean) => {
        if (stat.valueType !== 'boolean') return;
        if (!player) return;

        eventEmitter.emit({
            ChangeStatOfPlayer: {
                player_id: player.id,
                stat_id: stat.id,
                stat_type: 'boolean',
                stat_value: newValue.toString(),
            },
        });
    };

    // --- Remove ---
    const removeStat = (stat: Stat) => {
        if (!player) return;
        eventEmitter.emit({
            RemoveStatFromPlayer: {
                player_id: player.id,
                stat_id: stat.id,
            },
        });
    };

    // --- Add stat ---
    const newStatKey = ref('');
    const newStatType = ref<'string' | 'number' | 'boolean'>('string');
    const newStatValue = ref('');
    const addStatError = ref('');

    const addStat = () => {
        addStatError.value = '';
        if (!player) return;

        const trimmedKey = newStatKey.value.trim();
        if (!trimmedKey) {
            addStatError.value = 'Stat name is required.';
            return;
        }

        const duplicate = player.stats.some(s => s.key === trimmedKey);
        if (duplicate) {
            addStatError.value = `A stat named "${trimmedKey}" already exists.`;
            return;
        }

        let value: number | string | boolean = newStatValue.value;
        if (newStatType.value === 'number') value = parseFloat(newStatValue.value) || 0;
        if (newStatType.value === 'boolean') value = newStatValue.value === 'true';

        eventEmitter.emit({
            AddStatToPlayer: {
                player_id: player.id,
                stat_key: trimmedKey,
                stat_type: newStatType.value,
                stat_value: value.toString(),
            },
        });

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
