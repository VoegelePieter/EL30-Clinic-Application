# EL30-Clinic-Application

EL30 Assignment

Planning:
https://miro.com/app/board/uXjVKhtP49I=/

## Installation

### Prerequisites

Before you begin, ensure you have the following installed on your machine:

- [SurrealDB](https://surrealdb.com/docs/surrealdb/installation)
- [Rust](https://www.rust-lang.org/tools/install)

### Starting the Database

1. Open a terminal
2. Start SurrealDB with the following command:
   ```sh
   surreal start -u root -p root
   ```

### Starting the Backend

1. Open a new terminal
2. Navigate to the `backend` directory
   ```sh
   cd backend
   ```
3. Run the backend using Cargo
   ```sh
   cargo run
   ```

#### Configuring the Backend

You may want to change some configuration, like `port`, `doctor_amount`, `room_amount`, `opening_time`, `closing_time`, `break_time`. For this, navigate to the `server.toml` in `*/backend`

## Backend

### Testing

In the terminal, when inside of the `/backend` directory, run the command `cargo test --workspace -- --test-threads=1` to run all unit and integration tests
The reason for why the threads need to be limited to one is because the changes to the database are so frequent, and the database is so regularly cleared, that tests will not behave consistently otherwise.
As an example: _Test1_ starts its process, creates the empty mock database, creates some database entry, _Test2_ starts its process asynchronously, empties the mock database, and now the database entry that _Test1_ created is gone, causing _Test1_ to fail.

### API Documentation

The API can be most easily tested using the Postman Collection in the projects' root folder.

#### Base URL

```
http://127.0.0.1:8080/api
```

### Endpoints

---

### Patient Endpoints

#### Create Patient

- **URL**: `/patient`
- **Method**: `POST`
- **Description**: Creates a new patient.
- **Request Body**:
  ```json
  {
    "name": "John Doe",
    "phone_number": "+49 172 178923"
  }
  ```
- **Response**:
  ```json
  {
    "data": [
      {
        "id": {
          "tb": "patient",
          "id": {
            "String": "etz1z46uabcd2iykpyc8"
          }
        },
        "name": "John Doe",
        "phone_number": "+49 172 178923",
        "insurance_number": null
      }
    ]
  }
  ```

#### Get All Patients

- **URL**: `/patient`
- **Method**: `GET`
- **Description**: Retrieves all patients.
- **Response**:
  - A list of all patients

#### Get Patient by ID

- **URL**: `/patient/{id}`
- **Method**: `GET`
- **Description**: Retrieves a patient by ID.
- **Response**:
  - `200 OK` with the patient data
  - `404 Not Found` if the patient does not exist

#### Update Patient

- **URL**: `/patient/{id}`
- **Method**: `PUT`
- **Description**: Updates a patient by ID.
- **Valid Fields**: `name`, `phone_number`, `insurance_number`
- **Request Body**:
  ```json
  {
    "phone_number": "+69 231165",
    "insurance_number": "21935982"
  }
  ```
- **Response**:
  - `200 OK` on success with the patient data
  - `404 Not Found` if the patient does not exist

#### Delete Patient

- **URL**: `/patient/{id}`
- **Method**: `DELETE`
- **Description**: Deletes a patient by ID and all their appointments.
- **Response**:
  - `200 OK` on success
  - `404 Not Found` if the patient does not exist

---

### Appointment Endpoints

#### Create Appointment

- **URL**: `/appointment`
- **Method**: `POST`
- **Description**: Creates a new appointment.
- **Request Variables**:
  - `start_time` needs to be formatted as `YYYY-MM-DDTHH:MM:SS`
    - Invalid times that are outside of opening hours or overlap with other appointments are handled and don't need to be checked first
  - `appointment_type` can only be `quick_checkup`, `extensive_care`, or `surgery`
  - `patient_id` needs to be formatted as `patient:{$unique_id}`
  - `doctor` and `room_nr` are positive integers starting at 0
- **Request Body**:
  ```json
  {
    "start_time": "2015-11-15T09:00:00",
    "appointment_type": "surgery",
    "patient_id": "patient:etz1z46uabcd2iykpyc8",
    "doctor": 1,
    "room_nr": 1
  }
  ```
- **Response Variables**:
  - `end_time` is automatically calculated based on the provided `appointment_type`
- **Response**:
  ```json
  {
    "data": [
      {
        "id": {
          "tb": "appointment",
          "id": {
            "String": "l13i0kkl3j662o2ye3ql"
          }
        },
        "start_time": "2015-11-15T09:00:00",
        "end_time": "2015-11-15T11:00:00",
        "appointment_type": "surgery",
        "patient_id": "patient:etz1z46uabcd2iykpyc8",
        "doctor": 1,
        "room_nr": 1
      }
    ]
  }
  ```

#### Get All Appointments

- **URL**: `/appointment`
- **Method**: `GET`
- **Description**: Retrieves all appointments OR filtered appointments based on query parameters.
- **Optional Query Parameters**:
  - `filter`: The filter type (`day`, `month`, `patient_id`, `doctor`, `room_nr`)
  - `value`: The value for the filter (e.g. `2015-11-15`, `etz1z46uabcd2iykpyc8`, `0`)
- **Request**:
  - `http://localhost/api/appointment?filter=day&value=2015-11-15`
- **Response**:
  - `200 OK` with a list of appointments containing patient details

#### Get Appointment by ID

- **URL**: `/appointment/{id}`
- **Method**: `GET`
- **Description**: Retrieves an appointment by ID.
- **Response**:
  - `200 OK` with the appointment data containing patient details
    ```json
    {
      "data": {
        "id": {
          "tb": "appointment",
          "id": {
            "String": "8f1wm2ga1ih85unl2zcw"
          }
        },
        "start_time": "2015-11-16T08:00:00",
        "end_time": "2015-11-16T10:00:00",
        "appointment_type": "surgery",
        "patient": {
          "id": {
            "tb": "patient",
            "id": {
              "String": "6v1txo2ob7pyozxzlzbv"
            }
          },
          "name": "John Doe",
          "phone_number": "+69 420 178923",
          "insurance_number": null
        },
        "doctor": 1,
        "room_nr": 0
      }
    }
    ```
  - `404 Not Found` if the appointment does not exist

#### Update Appointment

- **URL**: `/appointment/{id}`
- **Method**: `PUT`
- **Description**: Updates the provided fields of an appointment by ID.
- **Request Variables**: Same as for appointment creation
- **Request Body**:

  - It's important to note that only the fields that are changed should be in the request

  ```json
  {
    "start_time": "2015-12-16T08:00:00",
    "appointment_type": "surgery",
    "doctor": 1,
    "room_nr": 0
  }
  ```

- **Response**:
  - `200 OK` on success with updated appointment
  - `404 Not Found` if the appointment does not exist

#### Delete Appointment

- **URL**: `/appointment/{id}`
- **Method**: `DELETE`
- **Description**: Deletes an appointment by ID.
- **Response**:
  - `200 OK` on success
  - `404 Not Found` if the appointment does not exist

#### Mass Reschedule Doctor

- **URL**: `/appointment/mass_reschedule`
- **Method**: `POST`
- **Description**: Automatically mass reschedules a doctor's appointments over a given timespan.
- **Request Body**:
  ```json
  {
    "doctor_id": 1,
    "start_date": "2023-10-01",
    "end_date": "2023-10-31"
  }
  ```
- **Response**:
  - `200 OK` with a list of all updated rescheduled appointments
  - `400 Bad Request` on validation error
