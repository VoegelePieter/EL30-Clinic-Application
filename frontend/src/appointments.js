import { showAlert, formatTime, calculateLength, capitalizeFirstLetter } from './utils.js';

/**
 * Clears the appointment form and resets validation.
 */
function clearAppointmentForm() {
    const createAppointmentForm = document.getElementById('create-appointment-form');
    createAppointmentForm.reset();
    document.getElementById('appointment-start-time').innerHTML = '<option value="" disabled selected>Select Start Time</option>';
    document.getElementById('room-number-select').innerHTML = '<option value="" disabled selected>Select a Room</option>';
    createAppointmentForm.classList.remove('was-validated');
}

/**
 * Fetches all appointments for a specific date and displays them.
 * @param {string} date - The selected date in format YYYY-MM-DD.
 */
export const fetchAndDisplayAppointments = async (date) => {
    console.log(`Fetching appointments for date: ${date}`);
    try {
        const response = await fetch(`http://127.0.0.1:8080/api/appointment?filter=day&value=${date}`, {
            method: 'GET',
            headers: { 'Content-Type': 'application/json' }
        });

        const appointmentsTableBody = document.getElementById('appointments-table-body');
        appointmentsTableBody.innerHTML = ''; // Clears the table before filling

        if (!response.ok) {
            console.error(`Error fetching appointments: ${response.statusText}`);
            appointmentsTableBody.innerHTML = '<tr><td colspan="8" class="text-center">No appointments to show.</td></tr>';
            document.getElementById('appointments-section').style.display = 'block';
            return;
        }

        let data;
        try {
            data = await response.json();
        } catch (parseError) {
            console.error('Invalid JSON format received from server.');
            appointmentsTableBody.innerHTML = '<tr><td colspan="8" class="text-center">No appointments to show.</td></tr>';
            document.getElementById('appointments-section').style.display = 'block';
            return;
        }

        const appointments = data.data;
        console.log(`Fetched Appointments:`, appointments);

        if (!appointments || appointments.length === 0) {
            appointmentsTableBody.innerHTML = '<tr><td colspan="8" class="text-center">No appointments to show.</td></tr>';
        } else {
            appointments.forEach(appointment => {
                const row = document.createElement('tr');

                row.innerHTML = `
                    <td>Doctor ${appointment.doctor}</td>
                    <td>${appointment.patient.name}</td>
                    <td>${capitalizeFirstLetter(appointment.appointment_type.replace('_', ' '))}</td>
                    <td>${formatTime(appointment.start_time)}</td>
                    <td>${formatTime(appointment.end_time)}</td>
                    <td>${calculateLength(appointment.start_time, appointment.end_time)} minutes</td>
                    <td>${appointment.room_nr}</td>
                    <td><button class="btn btn-danger btn-sm cancel-appointment-btn" data-appointment-id="${appointment.id.id.String}">Cancel</button></td>
                `;
                appointmentsTableBody.appendChild(row);
            });
        }

        // Displays the appointments section
        document.getElementById('appointments-section').style.display = 'block';
    } catch (error) {
        console.error(error);
        const appointmentsTableBody = document.getElementById('appointments-table-body');
        appointmentsTableBody.innerHTML = '<tr><td colspan="8" class="text-center">No appointments to show.</td></tr>';
        document.getElementById('appointments-section').style.display = 'block';
    }
};

/**
 * Creates a new appointment.
 * @param {Object} payload - The appointment data.
 * @param {HTMLElement} createAppointmentModal - The modal element for appointment creation.
 * @param {string} selectedDate - The currently selected date.
 */
export const createAppointment = async (payload, createAppointmentModal, selectedDate) => {
    console.log('Creating appointment with payload:', payload);
    try {
        const response = await fetch('http://127.0.0.1:8080/api/appointment', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(payload)
        });

        if (!response.ok) {
            const errorText = await response.text();
            throw new Error(`Failed to create appointment: ${errorText}`);
        }

        showAlert('Appointment created successfully.');
        clearAppointmentForm();
        const bootstrapModal = bootstrap.Modal.getInstance(createAppointmentModal);
        bootstrapModal.hide();

        // Refreshes the appointments list without changing the selected date
        fetchAndDisplayAppointments(selectedDate);
    } catch (error) {
        console.error(error);
        showAlert(`Error: ${error.message}`);
    }
};

/**
 * Deletes an appointment.
 * @param {string} appointmentId - The ID of the appointment.
 * @param {string} selectedDate - The currently selected date.
 */
export const deleteAppointment = async (appointmentId, selectedDate) => {
    console.log(`Cancelling appointment with ID: ${appointmentId}`);
    try {
        const response = await fetch(`http://127.0.0.1:8080/api/appointment/${appointmentId}`, {
            method: 'DELETE',
            headers: { 'Content-Type': 'application/json' }
        });

        if (!response.ok) {
            const errorText = await response.text();
            throw new Error(`Failed to cancel appointment: ${errorText}`);
        }

        showAlert('Appointment cancelled successfully.');
        // Refreshes the appointments list
        fetchAndDisplayAppointments(selectedDate);
    } catch (error) {
        console.error(error);
        showAlert(`Error: ${error.message}`);
    }
};
