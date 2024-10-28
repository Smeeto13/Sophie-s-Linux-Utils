/*
    rname, simple program to rename a file
    copyright (C) <2024> Sophie Smeeton

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License V3 as published by
    the Free Software Foundation.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use std::fs;
use std::path::Path;

fn main() {
    println!("Usage:\nrname (options) [current file path] (new file name/path)\
        \nOptions:\n-b, --backup default when no new name is given, creates a backup of the file nammed {{file}}.bak\
        in the current file location\n-m, --move will move the file to the path given\n-c --copy will create a coppy of the file at the given location");
}
