import { showAlert } from './utils.js';

/**
 * Fetches configuration data for doctors and rooms.
 * @returns {Promise<Object>} - An object containing doctorAmount and roomAmount.
 */
export const fetchConfig = async () => {
    try {
        const [doctorResponse, roomResponse] = await Promise.all([
            fetch('http://127.0.0.1:8080/api/config/doctor_amount', {
                method: 'GET',
                headers: { 'Content-Type': 'application/json' }
            }),
            fetch('http://127.0.0.1:8080/api/config/room_amount', {
                method: 'GET',
                headers: { 'Content-Type': 'application/json' }
            })
        ]);

        if (!doctorResponse.ok) {
            const errorText = await doctorResponse.text();
            throw new Error(`Error fetching doctor amount: ${errorText}`);
        }

        if (!roomResponse.ok) {
            const errorText = await roomResponse.text();
            throw new Error(`Error fetching room amount: ${errorText}`);
        }

        const doctorAmount = parseInt(await doctorResponse.text(), 10);
        const roomAmount = parseInt(await roomResponse.text(), 10);

        console.log(`Fetched Configuration: Doctors = ${doctorAmount}, Rooms = ${roomAmount}`);
        return { doctorAmount, roomAmount };
    } catch (error) {
        console.error(error);
        showAlert(`Failed to load configuration. ${error.message}`);
        return { doctorAmount: 0, roomAmount: 0 };
    }
};
