.timer on
.mode ascii
.echo on

-- Load the data
CREATE OR REPLACE TABLE measurements AS
        SELECT * FROM READ_CSV('./data/1M.csv', header=false, columns= {'station_name':'VARCHAR','measurement':'double'}, delim=';');

-- Run calculations
WITH src AS (SELECT station_name,
                    MIN(measurement) AS min_measurement,
                    CAST(AVG(measurement) AS DECIMAL(8,1)) AS mean_measurement,
                    MAX(measurement) AS max_measurement
            FROM measurements
            GROUP BY station_name)
    SELECT '{' ||
            ARRAY_TO_STRING(LIST_SORT(LIST(station_name || '=' || CONCAT_WS('/',min_measurement, mean_measurement, max_measurement))),', ') ||
            '}' AS "1BRC"
    FROM src;

.quit