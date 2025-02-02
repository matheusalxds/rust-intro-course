-- create the databases
CREATE DATABASE IF NOT EXISTS orbitware;

-- create the users for each database
CREATE USER 'math'@'%' IDENTIFIED BY 'math';
GRANT CREATE, ALTER, INDEX, LOCK TABLES, REFERENCES, UPDATE, DELETE, DROP, SELECT, INSERT ON `orbitware`.* TO 'math'@'%';

FLUSH PRIVILEGES;

-- Create the "products" table
CREATE TABLE products (
    product_code VARCHAR(255) PRIMARY KEY,
    color VARCHAR(255),
    product_name VARCHAR(255)
);

-- Create the "unique_identifiers" table
CREATE TABLE unique_identifiers (
    concatenated_string VARCHAR(255) PRIMARY KEY,
    product_code VARCHAR(255),
    color VARCHAR(255),
    product_name VARCHAR(255),
    warehouse VARCHAR(255),
    location VARCHAR(255),
    pcs INT,
    FOREIGN KEY (product_code) REFERENCES products(product_code)
);


-- Insert data into the "products" table
INSERT INTO products (product_code, color, product_name) VALUES
('806807071421', 'Peach', 'Smart thermostat'),
('806807071422', 'Aquamarine', 'Smart thermostat'),
('806807071423', 'Peach', 'Bamboo cutting board set'),
('806807071424', 'Aquamarine', 'Bamboo cutting board set'),
('806807071425', 'Peach', 'Noise-canceling headphones'),
('806807071426', 'Aquamarine', 'Noise-canceling headphones'),
('806807071427', 'Peach', 'Portable Bluetooth speaker'),
('806807071428', 'Aquamarine', 'Portable Bluetooth speaker'),
('806807071429', 'Peach', 'Reusable stainless steel water bottle'),
('806807071430', 'Aquamarine', 'Reusable stainless steel water bottle'),
('806807071431', 'Peach', 'Fitness tracker'),
('806807071432', 'Aquamarine', 'Fitness tracker'),
('806807071433', 'Peach', 'Aromatherapy diffuser'),
('806807071434', 'Aquamarine', 'Aromatherapy diffuser'),
('806807071435', 'Peach', 'Espresso machine'),
('806807071436', 'Aquamarine', 'Espresso machine');

-- Insert into "unique_identifiers" ---
INSERT INTO unique_identifiers (concatenated_string, product_code, color, product_name, warehouse, location, pcs) VALUES
('Aquamarine^Aromatherapy diffuser^HALA 5^M5-A-1', '806807071434', 'Aquamarine', 'Aromatherapy diffuser', 'HALA 5', 'M5-A-1', 20),
('Aquamarine^Aromatherapy diffuser^HALA 5^M5-A-7', '806807071434', 'Aquamarine', 'Aromatherapy diffuser', 'HALA 5', 'M5-A-7', 100),
('Aquamarine^Bamboo cutting board set^HALA 5^M5-A-2', '806807071424', 'Aquamarine', 'Bamboo cutting board set', 'HALA 5', 'M5-A-2', 100),
('Aquamarine^Espresso machine^HALA 5^M5-A-8', '806807071436', 'Aquamarine', 'Espresso machine', 'HALA 5', 'M5-A-8', 100),
('Aquamarine^Fitness tracker^HALA 5^M5-A-6', '806807071432', 'Aquamarine', 'Fitness tracker', 'HALA 5', 'M5-A-6', 100),
('Aquamarine^Noise-canceling headphones^HALA 5^M5-A-3', '806807071426', 'Aquamarine', 'Noise-canceling headphones', 'HALA 5', 'M5-A-3', 100),
('Aquamarine^Portable Bluetooth speaker^HALA 5^M5-A-4', '806807071428', 'Aquamarine', 'Portable Bluetooth speaker', 'HALA 5', 'M5-A-4', 100),
('Aquamarine^Reusable stainless steel water bottle^HALA 5^M5-A-5', '806807071430', 'Aquamarine', 'Reusable stainless steel water bottle', 'HALA 5', 'M5-A-5', 100),
('Aquamarine^Smart thermostat^HALA 5^M5-A-1', '806807071422', 'Aquamarine', 'Smart thermostat', 'HALA 5', 'M5-A-1', 100),
('Aquamarine^Smart thermostat^HALA 5^M5-A-10', '806807071422', 'Aquamarine', 'Smart thermostat', 'HALA 5', 'M5-A-10', 30);


