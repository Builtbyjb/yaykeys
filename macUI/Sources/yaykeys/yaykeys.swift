// The Swift Programming Language
// https://docs.swift.org/swift-book

import SwiftUI

struct ContentView: View {
    @Environment(\.managedObjectContext) private var viewContext
    var body: some View {
        Text("Welcome to Yaykeys").padding(64)
    }
}
