import { showAlert } from './utils.js';
import { populateDropdown } from './utils.js';

/**
 * Fetches all patients and populates the corresponding dropdowns.
 */
export const fetchAndPopulatePatients = async () => {
    try {
        const response = await fetch('http://127.0.0.1:8080/api/patient', {
            method: 'GET',
            headers: { 'Content-Type': 'application/json' }
        });

        if (!response.ok) {
            const errorText = await response.text();
            throw new Error(`Error fetching patients: ${errorText}`);
        }

        let data;
        try {
            data = await response.json();
        } catch (parseError) {
            throw new Error('Invalid JSON format received from server.');
        }

        const patients = data.data;
        console.log('Fetched Patients:', patients);

        populateDropdown(document.getElementById('appointment-patient-id'), patients, 'Select a Patient', true);
        populateDropdown(document.getElementById('patient-selection'), patients, 'Select a Patient', false);
    } catch (error) {
        console.error(error);
        showAlert(`Failed to load patients. ${error.message}`);
    }
};

/**
 * Fetches details of a specific patient and populates the update form.
 * @param {string} patientId - The ID of the patient.
 * @param {HTMLFormElement} updateForm - The update form.
 */
export const fetchPatientDetails = async (patientId, updateForm) => {
    console.log(`Fetching details for patient ID: ${patientId}`);
    try {
        const response = await fetch(`http://127.0.0.1:8080/api/patient/${patientId}`, {
            method: 'GET',
            headers: { 'Content-Type': 'application/json' }
        });

        if (!response.ok) {
            const errorText = await response.text();
            throw new Error(errorText || 'Patient not found.');
        }

        let data;
        try {
            data = await response.json();
        } catch (parseError) {
            throw new Error('Invalid JSON format received from server.');
        }

        const patient = data.data;
        console.log('Fetched Patient Details:', patient);

        document.getElementById('update-patient-name').value = patient.name;
        document.getElementById('update-patient-phone').value = patient.phone_number;

        updateForm.dataset.patientId = patientId;
        updateForm.classList.remove('d-none');
        updateForm.classList.remove('was-validated');
    } catch (error) {
        console.error(error);
        showAlert(`Failed to fetch patient details. ${error.message}`);
        updateForm.classList.add('d-none');
    }
};

/**
 * Creates a new patient.
 * @param {Object} payload - The patient data.
 */
export const createPatient = async (payload) => {
    console.log('Creating patient with payload:', payload);
    try {
        const response = await fetch('http://127.0.0.1:8080/api/patient', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload)
        });

        if (!response.ok) {
            const errorText = await response.text();
            throw new Error(`Failed to create patient: ${errorText}`);
        }

        showAlert('Patient created successfully.');
        fetchAndPopulatePatients();
    } catch (error) {
        console.error(error);
        showAlert(`Error: ${error.message}`);
    }
};

/**
 * Updates the data of an existing patient.
 * @param {string} patientId - The ID of the patient.
 * @param {Object} payload - The updated patient data.
 */
export const updatePatient = async (patientId, payload) => {
    console.log(`Updating patient ID ${patientId} with payload:`, payload);
    try {
        const response = await fetch(`http://127.0.0.1:8080/api/patient/${patientId}`, {
            method: 'PUT',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload)
        });

        if (!response.ok) {
            const errorText = await response.text();
            throw new Error(`Failed to update patient: ${errorText}`);
        }

        showAlert('Patient updated successfully.');
        fetchAndPopulatePatients();
    } catch (error) {
        console.error(error);
        showAlert(`Error: ${error.message}`);
    }
};

/**
 * Deletes a patient and all related appointments.
 * @param {string} patientId - The ID of the patient.
 */
export const deletePatient = async (patientId) => {
    console.log(`Deleting patient ID: ${patientId}`);
    try {
        const response = await fetch(`http://127.0.0.1:8080/api/patient/${patientId}`, {
            method: 'DELETE',
            headers: { 'Content-Type': 'application/json' }
        });

        if (!response.ok) {
            const errorText = await response.text();
            throw new Error(`Failed to delete patient: ${errorText}`);
        }

        showAlert('Patient deleted successfully.');
        fetchAndPopulatePatients();
    } catch (error) {
        console.error(error);
        showAlert(`Error: ${error.message}`);
    }
};
