import { showAlert } from './utils.js';
import { fetchConfig } from './config.js';

/**
 * Populates the doctor dropdown in a given select element.
 * @param {HTMLSelectElement} selectElement - The select element to populate.
 */
export const populateDoctorDropdown = async (selectElement) => {
    try {
        const { doctorAmount } = await fetchConfig();
        selectElement.innerHTML = '<option value="" disabled selected>Select a Doctor</option>';
        for (let i = 0; i <= doctorAmount; i++) {
            const option = document.createElement('option');
            option.value = i;
            option.textContent = `Doctor ${i}`;
            selectElement.appendChild(option);
        }
        console.log('Doctor Dropdown Populated');
    } catch (error) {
        console.error(error);
        showAlert(`Failed to load doctors. ${error.message}`);
    }
};

/**
 * Populates the doctor dropdown in the "Manage Doctors" modal.
 */
export const populateManageDoctorsDropdown = async () => {
    const manageDoctorSelect = document.getElementById('manage-doctor-id');
    await populateDoctorDropdown(manageDoctorSelect);
};

/**
 * Populates the doctor dropdown in the "Create Appointment" modal.
 */
export const populateDoctorsForAppointment = async () => {
    const doctorSelect = document.getElementById('doctor-id');
    await populateDoctorDropdown(doctorSelect);
};

/**
 * Performs mass rescheduling of a doctor's appointments during sickness.
 * @param {Object} payload - The reschedule data (doctor_id, start_date, end_date).
 */
export const massRescheduleDoctor = async (payload) => {
    console.log('Mass Reschedule Doctor with payload:', payload);
    try {
        const response = await fetch('http://127.0.0.1:8080/api/appointment/mass_reschedule', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload)
        });

        if (!response.ok) {
            const errorText = await response.text();
            console.log(JSON.stringify(payload));
            throw new Error(`Mass Reschedule failed: ${errorText}`);
        }

        showAlert('Doctor\'s appointments have been mass rescheduled successfully.');
    } catch (error) {
        console.error(error);
        showAlert(`Error: ${error.message}`);
    }
};
