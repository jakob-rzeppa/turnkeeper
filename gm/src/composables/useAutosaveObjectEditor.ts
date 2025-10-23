import { ref, watch } from 'vue'

/**
 * A composable to manage an editable object with autosave functionality.
 * It tracks changes to the fields of the object and provides methods to handle input and save changes.
 *
 * @param baseObjectCallback the callback function should return the baseObject to be edited (used by the watcher to track changes)
 */
export const useAutosaveObjectEditor = <T extends { [key: string]: string }>(
    baseObjectCallback: () => T,
    saveCallback: (newObject: T) => void,
) => {
    // This object holds the base object to be edited, but should not be modified directly (except via the watch)
    const baseObject = ref<T>(baseObjectCallback())

    // This object is the editable copy of the base object. It needs to be a deep copy to avoid modifying the base object directly.
    const editableObject = ref<T>(JSON.parse(JSON.stringify(baseObject.value)))
    const areEditableObjectFieldsChanged = ref<{ [K in keyof T]: boolean }>(
        {} as { [K in keyof T]: boolean },
    )
    Object.keys(editableObject.value).forEach((key) => {
        areEditableObjectFieldsChanged.value![key as keyof T] = false
    })

    watch(
        baseObjectCallback,
        (newBaseObject) => {
            baseObject.value = newBaseObject

            // If new records arrive from backend, update editableObject accordingly
            Object.keys(newBaseObject).forEach((key: keyof T) => {
                if (!(key in editableObject.value)) {
                    editableObject.value[key] = newBaseObject[key]
                }
            })

            // Recheck change tracking
            Object.keys(editableObject.value).forEach((key: keyof T) => {
                areEditableObjectFieldsChanged.value![key] =
                    baseObject.value[key] !== editableObject.value[key]
            })
        },
        { deep: true, immediate: false },
    )

    // This function handles input events for fields of the editable object
    const handleFieldInput = (field: keyof T, e: Event): void => {
        const target = e.target as HTMLInputElement
        const newValue = target.value

        editableObject.value[field] = newValue

        areEditableObjectFieldsChanged.value[field] = baseObject.value[field] !== newValue
    }

    const saveChanges = (): void => {
        const hasChanges = Object.values(areEditableObjectFieldsChanged.value).some(
            (changed) => changed,
        )
        if (hasChanges) {
            saveCallback(editableObject.value)

            // When saved, the backend is expected to send an event that updates the baseObject, which will trigger the watcher and update the baseObject accordingly.
        }
    }

    return {
        editableObject,
        areEditableObjectFieldsChanged,
        handleFieldInput,
        saveChanges,
    }
}
