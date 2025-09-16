let isGmConnected = false;

// Make sure, that only one Gm is connected at a time
export const authenticateGm = () => {
    if (isGmConnected) {
        throw new Error("Gm already connected");
    }

    isGmConnected = true;
};

export const disconnectGm = () => {
    isGmConnected = false;
};
