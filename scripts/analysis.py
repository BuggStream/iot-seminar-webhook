import csv

# Placeholder for the filename
filename = "placeholder.csv"

# Lists to store extracted data
timestamps = []
longitudes = []
latitudes = []

try:
    # Open the CSV file
    with open(filename, mode='r') as file:
        reader = csv.DictReader(file)  # Read file as a dictionary

        for row in reader:
            # Extract and validate timestamp
            timestamp = row.get("received_at", "NA")
            if timestamp and timestamp != "NA":
                timestamps.append(timestamp)

            # Extract and validate longitude
            try:
                longitude = float(row.get("rx_lng", "NA"))
                longitudes.append(longitude)
            except (ValueError, TypeError):
                # Skip invalid longitude
                longitudes.append(None)

            # Extract and validate latitude
            try:
                latitude = float(row.get("rx_lat", "NA"))
                latitudes.append(latitude)
            except (ValueError, TypeError):
                # Skip invalid latitude
                latitudes.append(None)

    # Output the results
    print("Timestamps:", timestamps)
    print("Longitudes:", longitudes)
    print("Latitudes:", latitudes)

except FileNotFoundError:
    print(f"Error: File '{filename}' not found.")
except Exception as e:
    print(f"An unexpected error occurred: {e}")
