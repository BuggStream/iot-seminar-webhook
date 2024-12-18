import pandas as pd
import numpy as np


# Function to calculate haversine distance
def haversine(lat1, lon1, lat2, lon2):
    # Convert latitude and longitude from degrees to radians
    lat1, lon1, lat2, lon2 = map(np.radians, [lat1, lon1, lat2, lon2])

    # Haversine formula
    dlat = lat2 - lat1
    dlon = lon2 - lon1
    a = np.sin(dlat / 2) ** 2 + np.cos(lat1) * np.cos(lat2) * np.sin(dlon / 2) ** 2
    c = 2 * np.arctan2(np.sqrt(a), np.sqrt(1 - a))
    r = 6371  # Earth's radius in kilometers
    return r * c


def triangulate_position_with_weights(filename):
    try:
        # Load the CSV file, handling NA values
        data = pd.read_csv(filename, na_values="NA")

        # Filter relevant columns and drop rows with missing data in required fields
        data = data[['message_id', 'rx_lat', 'rx_lng', 'rssi', 'snr']].dropna()

        # Group by message_id to process each message separately
        triangulated_positions = []

        for message_id, group in data.groupby('message_id'):
            # Extract values for the current message_id
            rx_lats = group['rx_lat'].to_numpy()
            rx_lngs = group['rx_lng'].to_numpy()
            rssis = group['rssi'].to_numpy()
            snrs = group['snr'].to_numpy()

            # Calculate weights based on RSSI and SNR
            # Normalize weights to sum up to 1 for each message_id
            weights = np.exp(rssis / 10) * (snrs + 10)  # Adding 1 to SNR to ensure non-zero contribution
            normalized_weights = weights / np.sum(weights)

            # Calculate weighted average for latitude and longitude
            weighted_lat = np.sum(rx_lats * normalized_weights)
            weighted_lng = np.sum(rx_lngs * normalized_weights)

            triangulated_positions.append({
                'message_id': message_id,
                'device_lat': weighted_lat,
                'device_lng': weighted_lng
            })

        # Convert the triangulated positions into a DataFrame for easier use
        triangulated_df = pd.DataFrame(triangulated_positions)

        print(triangulated_df)
        return triangulated_df

    except Exception as e:
        print(f"An error occurred: {e}")


# Function to find closest positions to a reference point, including message_id
def find_closest_positions(triangulated_df, ref_lat, ref_lng):
    try:
        # Add a column for the haversine distance to the reference point
        triangulated_df['distance'] = triangulated_df.apply(
            lambda row: haversine(ref_lat, ref_lng, row['device_lat'], row['device_lng']),
            axis=1
        )

        # Sort the DataFrame by distance in ascending order
        sorted_df = triangulated_df.sort_values(by='distance').reset_index(drop=True)

        # Display the sorted DataFrame with message_id, device_lat, device_lng, and distance
        print(sorted_df[['message_id', 'device_lat', 'device_lng', 'distance']].to_string())
        return sorted_df[['message_id', 'device_lat', 'device_lng', 'distance']]

    except Exception as e:
        print(f"An error occurred while finding closest positions: {e}")


# Example usage
filename = "datasets/friday.csv"  # Replace with your actual file path
triangulated_positions = triangulate_position_with_weights(filename)

# Step 2: Find closest positions to the reference point (51.998, 4.374)
reference_lat = 51.998
reference_lng = 4.374
closest_positions = find_closest_positions(triangulated_positions, reference_lat, reference_lng)
