#!/usr/bin/python3
#
# Copyright (c) 2020 Fred Morris, Tacoma WA 98445 USA
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#       http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import unittest
import wtrack_base

ONE_RECORD = """1595096135.744034\t2.437GHz\t44\t0\t5\t1c:b7:2c:7c:15:98\tf4:f5:24:dd:83:9d\t0\t2535385091\t1\t\\x82\\x84\\x8b\\x96$0Hl\t3\t\\x08\t42\t\\x00\t45\t\\xfc\\x19\\x1b\\xff\\xff\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\t47\t\\x00\t48\t\\x01\\x00\\x00\\x0f\\xac\\x04\\x01\\x00\\x00\\x0f\\xac\\x04\\x01\\x00\\x00\\x0f\\xac\\x02\\x0c\\x00\t50\t\\x0c\\x12\\x18`\t61\t\\x08\\x08\\x11\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\\x00\t221+\t\\x00\\t\\x00\\x10\\x18\\x02\\x01\\x10,\\x00\\x00\\x00\\x18\\x00P\\xf2\\x02\\x01\\x01\\x80\\x00\\x03\\xa4\\x00\\x00'\\xa4\\x00\\x00BC^\\x00b2/\\x00\t999\t\n"""

DATA_RECORD = """1595273999.7138083\t2.442GHz\t56\t2\t0\t0c:51:01:e4:da:2c\t28:ff:3c:a0:f0:7c"""

BAD_RECORD = """1595096135.744034\t2.437GHz\t44\t0\t5\t1c:b7:2c:7c:15:98"""

class TestBasicFunctionality(unittest.TestCase):
    """Tests of BaseDevice basic functionality."""
    def setUp(self):
        self.observation = wtrack_base.BaseDevice(ONE_RECORD)
        return
    
    def test_object_creation(self):
        """BaseDevice successfully instantiated."""
        self.assertEqual(self.observation.record, ONE_RECORD)
        return
    
    def test_timestamp(self):
        """timestamp field"""
        self.assertEqual(self.observation.timestamp, float(ONE_RECORD.split('\t')[0]))
        return
    
    def test_frequency(self):
        """frequency field"""
        self.assertEqual(self.observation.frequency, ONE_RECORD.split('\t')[1])
        return

    def test_signal(self):
        """signal strength field"""
        self.assertEqual(self.observation.signal, int(ONE_RECORD.split('\t')[2]))
        return

    def test_type(self):
        """type field"""
        self.assertEqual(self.observation.type, int(ONE_RECORD.split('\t')[3]))
        return

    def test_subtype(self):
        """subtype field"""
        self.assertEqual(self.observation.subtype, int(ONE_RECORD.split('\t')[4]))
        return
    
    def test_packet_type(self):
        type, subtype = self.observation.packet_type
        self.assertEqual(type, int(ONE_RECORD.split('\t')[3]))
        self.assertEqual(subtype, int(ONE_RECORD.split('\t')[4]))
        return

    def test_src(self):
        """src field"""
        self.assertEqual(self.observation.src, ONE_RECORD.split('\t')[5])
        return

    def test_dest(self):
        """dest field"""
        self.assertEqual(self.observation.dest, ONE_RECORD.split('\t')[6])
        return
    
    def test_attr(self):
        """attribute retrieval"""
        self.assertEqual(self.observation.attr("42"), "\\x00")
        self.assertEqual(self.observation.attr("foo"), None)
        return
    
    def test_empty_last_attr(self):
        """the last attribute should be present even if empty"""
        self.assertEqual(self.observation.attr("999"), "")
        return

    def test_station(self):
        """station attribute"""
        self.assertEqual(self.observation.station, ONE_RECORD.split('\t')[8])
        return

class TestValidRecord(unittest.TestCase):
    
    def test_valid_record(self):
        observation = wtrack_base.BaseDevice(ONE_RECORD)
        self.assertTrue(observation.valid())
        return
    
    def test_data_record(self):
        observation = wtrack_base.BaseDevice(DATA_RECORD)
        self.assertTrue(observation.valid())
        return

    def test_invalid_record(self):
        observation = wtrack_base.BaseDevice(BAD_RECORD)
        self.assertFalse(observation.valid())
        return
    
    def test_invalid_timestamp(self):
        rec = ONE_RECORD.split('\t')
        rec[0] = 'foo'
        observation = wtrack_base.BaseDevice('\t'.join(rec))
        self.assertEqual(observation.timestamp, -1.0)
        return

    def test_invalid_type(self):
        rec = ONE_RECORD.split('\t')
        rec[3] = 'foo'
        observation = wtrack_base.BaseDevice('\t'.join(rec))
        self.assertEqual(observation.type, None)
        return

class TestMetadata(unittest.TestCase):
    
    def test_hex_removal(self):
        rec = ONE_RECORD.split('\t')
        rec[8] = '\\x00\\x00foo!\\x35'
        observation = wtrack_base.BaseDevice('\t'.join(rec))
        self.assertEqual(observation.station, '..foo!.')
        return
    
    def test_ap(self):
        observation = wtrack_base.BaseDevice(ONE_RECORD)
        self.assertFalse(observation.ap)
        rec = ONE_RECORD.split('\t')
        rec[4] = '8'
        observation = wtrack_base.BaseDevice('\t'.join(rec))
        self.assertTrue(observation.ap)
        return
    
if __name__ == '__main__':
    unittest.main(verbosity=2)
    
