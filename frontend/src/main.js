import { showAlert, handleFormValidation, populateDropdown, populateRoomDropdown, populateStartTimeDropdown } from './utils.js';
import { fetchConfig } from './config.js';
import { fetchAndPopulatePatients, fetchPatientDetails, createPatient, updatePatient, deletePatient } from './patients.js';
import { populateManageDoctorsDropdown, populateDoctorsForAppointment, massRescheduleDoctor } from './doctors.js';
import { fetchAndDisplayAppointments, createAppointment, deleteAppointment } from './appointments.js';

/**
 * Initializes all event listeners and performs the initial setup.
 */
document.addEventListener('DOMContentLoaded', () => {
    // Initialize Modal Elements
    const managePatientsModal = document.getElementById('managePatientsModal');
    const createAppointmentModal = document.getElementById('createAppointmentModal');
    const manageDoctorsModal = document.getElementById('manageDoctorsModal');
    const confirmCancelModalElement = document.getElementById('confirmCancelModal');
    const confirmCancelModal = new bootstrap.Modal(confirmCancelModalElement, { keyboard: false });

    // Forms
    const createPatientForm = document.getElementById('create-patient-form');
    const patientSelection = document.getElementById('patient-selection');
    const updateDeletePatientForm = document.getElementById('update-delete-patient-form');
    const deletePatientBtn = document.getElementById('delete-patient-btn');

    const createAppointmentForm = document.getElementById('create-appointment-form');
    const appointmentTypeSelect = document.getElementById('appointment-type');

    const manageDoctorsForm = document.getElementById('manage-doctors-form');

    // Date and Appointments Elements
    const selectDayInput = document.getElementById('select-day');
    const appointmentsSection = document.getElementById('appointments-section');
    const selectedDateSpan = document.getElementById('selected-date');
    const appointmentsTableBody = document.getElementById('appointments-table-body');

    // Confirmation Modal Elements
    const confirmCancelBtn = document.getElementById('confirm-cancel-btn');

    let appointmentIdToCancel = null;

    /**
     * Sets the default date to today and loads the corresponding appointments.
     */
    const setDefaultDate = () => {
        const today = new Date().toISOString().split('T')[0];
        selectDayInput.value = today;
        selectedDateSpan.textContent = today;
        fetchAndDisplayAppointments(today);
    };

    /**
     * Clears the appointment form and resets validation.
     */
    const clearAppointmentForm = () => {
        createAppointmentForm.reset();
        document.getElementById('appointment-start-time').innerHTML = '<option value="" disabled selected>Select Start Time</option>';
        document.getElementById('room-number-select').innerHTML = '<option value="" disabled selected>Select a Room</option>';
        createAppointmentForm.classList.remove('was-validated');
    };

    // Event Listeners

    // Opens and fills the "Create Appointment" Modal
    createAppointmentModal.addEventListener('show.bs.modal', async () => {
        await fetchAndPopulatePatients();
        await populateDoctorsForAppointment();
        await fetchConfig().then(config => {
            const roomSelect = document.getElementById('room-number-select');
            populateRoomDropdown(roomSelect, config.roomAmount);
            const startTimeSelect = document.getElementById('appointment-start-time');
            populateStartTimeDropdown(startTimeSelect);
        });
    });

    // Opens and fills the "Manage Patients" Modal
    managePatientsModal.addEventListener('show.bs.modal', fetchAndPopulatePatients);

    // Opens and fills the "Manage Doctors" Modal
    manageDoctorsModal.addEventListener('show.bs.modal', populateManageDoctorsDropdown);

    // Generates available time slots based on the selected appointment type
    appointmentTypeSelect.addEventListener('change', () => {
        // Since the start time is now independent of the appointment type, we do nothing here.
        // Alternatively, you could call a function here if further adjustments are needed.
    });

    // Creates a new patient
    createPatientForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        if (!handleFormValidation(createPatientForm)) return;

        const name = document.getElementById('new-patient-name').value.trim();
        const phoneNumber = document.getElementById('new-patient-phone').value.trim();

        const payload = { name, phone_number: phoneNumber };
        await createPatient(payload);
    });

    // Handles the selection of a patient for updating or deletion
    patientSelection.addEventListener('change', (e) => {
        const selectedValue = e.target.value;
        if (!selectedValue) {
            updateDeletePatientForm.classList.add('d-none');
            return;
        }

        const patientId = selectedValue.startsWith('patient:') ? selectedValue.split(':')[1] : selectedValue;
        fetchPatientDetails(patientId, updateDeletePatientForm);
    });

    // Updates a patient
    updateDeletePatientForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        if (!handleFormValidation(updateDeletePatientForm)) return;

        const patientId = updateDeletePatientForm.dataset.patientId;
        const updatedName = document.getElementById('update-patient-name').value.trim();
        const updatedPhone = document.getElementById('update-patient-phone').value.trim();

        const payload = { name: updatedName, phone_number: updatedPhone };
        await updatePatient(patientId, payload);
    });

    // Deletes a patient
    deletePatientBtn.addEventListener('click', async () => {
        const patientId = updateDeletePatientForm.dataset.patientId;
        if (!patientId) {
            showAlert('No patient selected for deletion.');
            return;
        }

        if (!confirm('Are you sure you want to delete this patient and all their appointments?')) return;

        await deletePatient(patientId);
    });

    // Creates a new appointment
    createAppointmentForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        if (!handleFormValidation(createAppointmentForm)) return;

        const selectedDay = document.getElementById('appointment-day').value;
        const selectedTime = document.getElementById('appointment-start-time').value;
        const startTime = `${selectedDay}T${selectedTime}`;

        const appointmentType = document.getElementById('appointment-type').value;
        const patientId = document.getElementById('appointment-patient-id').value;
        const doctorId = document.getElementById('doctor-id').value;
        const roomNumber = document.getElementById('room-number-select').value;

        const payload = {
            start_time: startTime,
            appointment_type: appointmentType,
            patient_id: patientId,
            doctor: parseInt(doctorId, 10),
            room_nr: parseInt(roomNumber, 10)
        };

        await createAppointment(payload, createAppointmentModal, selectDayInput.value);
    });

    // Performs the mass rescheduling for a doctor
    manageDoctorsForm.addEventListener('submit', async (e) => {
        e.preventDefault();
        if (!handleFormValidation(manageDoctorsForm)) return;

        const doctorId = document.getElementById('manage-doctor-id').value;
        const startDate = document.getElementById('sickness-start-date').value;
        const endDate = document.getElementById('sickness-end-date').value;

        if (new Date(startDate) > new Date(endDate)) {
            showAlert('Start Date cannot be after End Date.');
            return;
        }

        const payload = {
            doctor_id: parseInt(doctorId, 10),
            start_date: startDate,
            end_date: endDate
        };

        await massRescheduleDoctor(payload);

        // Refreshes the appointments list
        fetchAndDisplayAppointments(selectDayInput.value);
    });

    // Handles the date selection and loads the corresponding appointments
    selectDayInput.addEventListener('change', (e) => {
        const selectedDate = e.target.value;
        console.log(`Selected Date Changed to: ${selectedDate}`);
        if (selectedDate) {
            selectedDateSpan.textContent = selectedDate;
            fetchAndDisplayAppointments(selectedDate);
        } else {
            appointmentsSection.style.display = 'none';
        }
    });

    // Navigates to the next day and loads the appointments
    document.getElementById('next-day-btn').addEventListener('click', () => {
        const currentDate = new Date(selectDayInput.value);
        currentDate.setDate(currentDate.getDate() + 1);
        const newDate = currentDate.toISOString().split('T')[0];
        selectDayInput.value = newDate;
        selectedDateSpan.textContent = newDate;
        fetchAndDisplayAppointments(newDate);
    });

    // Navigates to the previous day and loads the appointments
    document.getElementById('prev-day-btn').addEventListener('click', () => {
        const currentDate = new Date(selectDayInput.value);
        currentDate.setDate(currentDate.getDate() - 1);
        const newDate = currentDate.toISOString().split('T')[0];
        selectDayInput.value = newDate;
        selectedDateSpan.textContent = newDate;
        fetchAndDisplayAppointments(newDate);
    });

    // Initial Setup
    setDefaultDate();

    // Handles the clicking on the "Cancel" button in the appointments
    appointmentsTableBody.addEventListener('click', (e) => {
        if (e.target && e.target.matches('button.cancel-appointment-btn')) {
            appointmentIdToCancel = e.target.dataset.appointmentId;
            console.log(`Selected Appointment ID for Cancellation: ${appointmentIdToCancel}`);
            confirmCancelModal.show();
        }
    });

    // Confirms the cancellation of an appointment
    confirmCancelBtn.addEventListener('click', async () => {
        if (!appointmentIdToCancel) {
            showAlert('No appointment selected for cancellation.');
            confirmCancelModal.hide();
            return;
        }

        await deleteAppointment(appointmentIdToCancel, selectDayInput.value);
        confirmCancelModal.hide();
        appointmentIdToCancel = null;
    });
});
