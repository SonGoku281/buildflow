-- Seed data: Materials catalog
-- Run this AFTER 001_initial_schema.sql

INSERT INTO materials (category, brand, product, grade, size, type, unit, price, region) VALUES
-- Cement
('cement', 'UltraTech', NULL, 'OPC 53', NULL, NULL, '50kg bag', 420, 'odisha'),
('cement', 'UltraTech', NULL, 'PPC', NULL, NULL, '50kg bag', 400, 'odisha'),
('cement', 'ACC', NULL, 'OPC 53', NULL, NULL, '50kg bag', 415, 'odisha'),
('cement', 'Birla White', NULL, 'OPC 53', NULL, NULL, '50kg bag', 425, 'odisha'),
('cement', 'JK Cement', NULL, 'OPC 53', NULL, NULL, '50kg bag', 410, 'odisha'),

-- Steel
('steel', 'Tata Tiscon', NULL, 'Fe500D', NULL, NULL, 'per kg', 82, 'odisha'),
('steel', 'Tata Tiscon', NULL, 'Fe550D', NULL, NULL, 'per kg', 88, 'odisha'),
('steel', 'SAIL', NULL, 'Fe500', NULL, NULL, 'per kg', 78, 'odisha'),
('steel', 'JSW Steel', NULL, 'Fe500D', NULL, NULL, 'per kg', 80, 'odisha'),

-- Bricks
('bricks', NULL, NULL, 'First Class', NULL, 'Red Clay Brick', 'per piece', 12, 'odisha'),
('bricks', NULL, NULL, 'First Class', NULL, 'Fly Ash Brick', 'per piece', 8, 'odisha'),
('bricks', NULL, NULL, 'Standard', NULL, 'Concrete Block', 'per piece', 25, 'odisha'),
('bricks', NULL, NULL, 'Second Class', NULL, 'Red Clay Brick', 'per piece', 9, 'odisha'),

-- Sand
('sand', NULL, NULL, 'Zone II', NULL, 'River Sand', 'per cft', 65, 'odisha'),
('sand', NULL, NULL, 'Zone II', NULL, 'M-Sand', 'per cft', 50, 'odisha'),
('sand', NULL, NULL, 'Standard', NULL, 'Crusher Sand', 'per cft', 40, 'odisha'),
('sand', NULL, NULL, 'Fine', NULL, 'Concrete Sand', 'per cft', 55, 'odisha'),

-- Aggregate
('aggregate', NULL, NULL, NULL, '20mm', NULL, 'per cft', 90, 'odisha'),
('aggregate', NULL, NULL, NULL, '10mm', NULL, 'per cft', 95, 'odisha'),
('aggregate', NULL, NULL, NULL, '4mm (Fine)', NULL, 'per cft', 85, 'odisha'),
('aggregate', NULL, NULL, NULL, '40mm (Coarse)', NULL, 'per cft', 88, 'odisha'),

-- Waterproofing
('waterproofing', 'Dr. Fixit', 'Surestop', NULL, NULL, NULL, 'per kg', 180, 'odisha'),
('waterproofing', 'Nippo', 'Neptune', NULL, NULL, NULL, 'per liter', 160, 'odisha'),
('waterproofing', 'Sika', 'Waterstop', NULL, NULL, NULL, 'per kg', 200, 'odisha'),
('waterproofing', 'Nippon Paint', 'Waterblock', NULL, NULL, NULL, 'per liter', 170, 'odisha'),

-- Paint
('paint', 'Asian Paints', 'Royale Play', NULL, NULL, 'premium', 'per liter', 450, 'odisha'),
('paint', 'Asian Paints', 'Tractor Emulsion', NULL, NULL, 'economy', 'per liter', 220, 'odisha'),
('paint', 'Berger', 'Silk Touch', NULL, NULL, 'standard', 'per liter', 320, 'odisha'),
('paint', 'Nippon Paint', 'WeatherShield', NULL, NULL, 'standard', 'per liter', 280, 'odisha'),
('paint', 'Asian Paints', 'Berita', NULL, NULL, 'economy', 'per liter', 180, 'odisha'),

-- Flooring
('flooring', NULL, NULL, NULL, NULL, 'Vitrified Tiles (economy)', 'per sq ft', 35, 'odisha'),
('flooring', NULL, NULL, NULL, NULL, 'Porcelain Tiles (standard)', 'per sq ft', 65, 'odisha'),
('flooring', NULL, NULL, NULL, NULL, 'Marble (standard)', 'per sq ft', 120, 'odisha'),
('flooring', NULL, NULL, NULL, NULL, 'Granite (standard)', 'per sq ft', 90, 'odisha'),
('flooring', NULL, NULL, NULL, NULL, 'Wooden Flooring (premium)', 'per sq ft', 180, 'odisha'),
('flooring', NULL, NULL, NULL, NULL, 'Kotah Stone (standard)', 'per sq ft', 75, 'odisha'),
('flooring', NULL, NULL, NULL, NULL, 'Terrazzo (standard)', 'per sq ft', 85, 'odisha'),

-- Electrical
('electrical', 'Finolex', 'Cu Wire 1mm', NULL, NULL, NULL, 'per meter', 18, 'odisha'),
('electrical', 'Finolex', 'Cu Wire 2.5mm', NULL, NULL, NULL, 'per meter', 45, 'odisha'),
('electrical', 'Finolex', 'Cu Wire 4mm', NULL, NULL, NULL, 'per meter', 75, 'odisha'),
('electrical', 'Havells', 'Switch Board', NULL, NULL, NULL, 'per piece', 180, 'odisha'),
('electrical', 'Anchor by Schneider', 'MCB', NULL, NULL, NULL, 'per piece', 250, 'odisha'),
('electrical', 'Lovely', 'Conduit Pipe', NULL, NULL, NULL, 'per piece', 35, 'odisha'),

-- Plumbing
('plumbing', 'Astral', 'CPVC Pipe 20mm', NULL, NULL, NULL, 'per piece', 45, 'odisha'),
('plumbing', 'Astral', 'CPVC Pipe 25mm', NULL, NULL, NULL, 'per piece', 75, 'odisha'),
('plumbing', 'Astral', 'CPVC Pipe 40mm', NULL, NULL, NULL, 'per piece', 150, 'odisha'),
('plumbing', 'Jaquar', 'Wash Basin', NULL, NULL, NULL, 'per piece', 2800, 'odisha'),
('plumbing', 'Jaquar', 'Toilet Seat', NULL, NULL, NULL, 'per piece', 4500, 'odisha'),
('plumbing', 'Cera', 'Shower Mixer', NULL, NULL, NULL, 'per piece', 3500, 'odisha'),

-- Windows & Doors
('windows_doors', NULL, NULL, NULL, '800x2100mm', 'Wooden Door (Teak)', 'per piece', 8000, 'odisha'),
('windows_doors', NULL, NULL, NULL, '1200x1200mm', 'PVC Window', 'per sq ft', 180, 'odisha'),
('windows_doors', NULL, NULL, NULL, '1200x1200mm', 'Aluminum Window', 'per sq ft', 220, 'odisha'),
('windows_doors', NULL, NULL, NULL, '800x2100mm', 'Wooden Door (Sal)', 'per piece', 5000, 'odisha'),

-- Roofing
('roofing', NULL, NULL, NULL, NULL, 'CI Sheets', 'per sq ft', 120, 'odisha'),
('roofing', NULL, NULL, NULL, NULL, 'WPC Decking', 'per sq ft', 180, 'odisha'),
('roofing', NULL, NULL, NULL, NULL, 'Waterproofing Membrane', 'per sq m', 85, 'odisha'),
('roofing', NULL, NULL, NULL, NULL, 'Colorbond Sheet', 'per sq ft', 140, 'odisha'),

-- Finishing
('finishing', 'Asian Paints', 'Putty', NULL, NULL, NULL, 'per kg', 35, 'odisha'),
('finishing', 'Asian Paints', 'Primer', NULL, NULL, NULL, 'per liter', 120, 'odisha'),
('finishing', NULL, NULL, NULL, NULL, 'PU Paint', 'per liter', 600, 'odisha'),
('finishing', NULL, NULL, NULL, NULL, 'Wallpaper', 'per roll', 800, 'odisha'),
('finishing', NULL, NULL, NULL, NULL, 'Tile Grout', 'per kg', 45, 'odisha');
