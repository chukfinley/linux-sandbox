from imagededup.methods import CNN
from imagededup.utils import plot_duplicates

# Verzeichnis definieren, das die Bilder enthält
input_dir = '../examplefiles/images'

# CNN-Methode initialisieren
cnn = CNN()

# Duplikate finden
duplicates = cnn.find_duplicates(image_dir=input_dir)

# Für jedes Bild mit Duplikaten eine HTML-Datei erstellen
for filename, duplicate_files in duplicates.items():
    if duplicate_files:  # Prüfen, ob Duplikate vorhanden sind
        output_html = f'duplicates_of_{filename}.html'  # Dateiname für die Duplikate des spezifischen Bildes
        plot_duplicates(image_dir=input_dir, duplicate_map={filename: duplicate_files}, filename=filename)
