import { ref, watch } from 'vue';

/**
 * A composable to manage an editable object with autosave functionality.
 * It tracks changes to the fields of the object and provides methods to handle input and save changes.
 *
 * @param baseObjectCallback the callback function should return the baseObject to be edited (used by the watcher to track changes)
 */
export const useAutosaveObject = <T extends object>(
    baseObjectCallback: () => T,
    saveCallback: (newObject: T) => void,
) => {
    // This object holds the base object to be edited, but should not be modified directly (except via the watch)
    const baseObject = ref<T>(baseObjectCallback());

    // This object is the editable copy of the base object. It needs to be a deep copy to avoid modifying the base object directly.
    const editableObject = ref<T>(JSON.parse(JSON.stringify(baseObject.value)));
    const isEditableObjectChanged = ref(false);

    // Watch for changes in the editableObject to track if any field has changed compared to the baseObject
    watch(
        editableObject,
        (newEditableObject) => {
            // Check if any field has changed compared to the baseObject (deep comparison)
            isEditableObjectChanged.value =
                JSON.stringify(baseObject.value) !== JSON.stringify(newEditableObject);
        },
        { deep: true },
    );

    // Watch for changes in the baseObjectCallback to update the baseObject and editableObject accordingly
    watch(
        baseObjectCallback,
        (newBaseObject) => {
            baseObject.value = newBaseObject;

            // If new records arrive from backend, update editableObject accordingly
            editableObject.value = JSON.parse(JSON.stringify(newBaseObject));

            // Recheck change tracking (deep comparison)
            isEditableObjectChanged.value =
                JSON.stringify(baseObject.value) !== JSON.stringify(editableObject.value);
        },
        { deep: true, immediate: false },
    );

    const saveChanges = (): void => {
        if (isEditableObjectChanged.value) {
            saveCallback(editableObject.value);

            // When saved, the backend is expected to send an event that updates the baseObject, which will trigger the watcher and update the baseObject accordingly.
        }
    };

    return {
        baseObject,
        editableObject,
        isEditableObjectChanged,
        saveChanges,
    };
};
