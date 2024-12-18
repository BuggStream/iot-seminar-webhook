import pandas as pd

# Load the CSV file into a DataFrame
file_path = "datasets/friday.csv"  # Replace with your actual file path
data = pd.read_csv(file_path)

# Group data by message_id
grouped = data.groupby("message_id")

# Prepare a DataFrame to store results
triangulated_positions = []

centroid_lat_sum = 0
centroid_lng_sum = 0
message_counter = 0

for message_id, group in grouped:
    # Extract rx_lat and rx_lng for the current message_id

    rx_lats = group["rx_lat"]
    rx_lngs = group["rx_lng"]

    # Calculate the centroid
    centroid_lat = rx_lats.mean()
    centroid_lng = rx_lngs.mean()

    # Store the result
    triangulated_positions.append({"message_id": message_id, "device_lat": centroid_lat, "device_lng": centroid_lng})

    centroid_lat_sum += centroid_lat
    centroid_lng_sum += centroid_lng

    message_counter += 1

# Create a new DataFrame with triangulated positions
results_df = pd.DataFrame(triangulated_positions)

# Save the results to a CSV file or display them
results_df.to_csv("triangulated_positions.csv", index=False)
print(results_df)
# print("lat:%f long%f", centroid_lat_sum/message_counter, centroid_lng_sum/message_counter)
print(f"lat:{centroid_lat_sum / message_counter:.6f} long:{centroid_lng_sum / message_counter:.6f}")
