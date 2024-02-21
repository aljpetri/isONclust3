use crate::structs::{GtfEntry, FastaRecord, Coord_obj};
use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::str::FromStr;
use rustc_hash::{FxHashMap, FxHasher};
use bio::io::gff;
use bio::io::gff::{GffType, Record};
use bio::io::gff::GffType::GFF3;
use rayon::prelude::*;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::path::Path;
use bio::io::fasta;
use bio::io::fasta::FastaRead;
extern crate rayon;
use crate::generate_sorted_fastq_new_version;
use crate::clustering;


fn get_coords(rec: Record, mut coords: &mut FxHashMap<&str,FxHashMap<i32,Vec<Coord_obj>>>,gene_id:i32){
    if rec.feature_type() == "exon"{

        /*else{
            let mut coord_vec=vec![];
            coord_vec.push(Coord_obj{startpos: *rec.start(),endpos: *rec.end() });
            coords.insert(gene_id, coord_vec);
        }*/
    }
}




fn parse_fasta_and_gen_clusters(fasta_path: Option<&str>, coords: FxHashMap<String,FxHashMap<i32,Vec<Coord_obj>>>,clusters: &mut FxHashMap<i32, Vec<i32>>, init_cluster_map: &mut FxHashMap<u64, Vec<i32>>){
    println!("parse_fasta");
    let path=fasta_path.unwrap();
    let mut reader = fasta::Reader::from_file(Path::new(path)).expect("We expect the file to exist");
    //let mut reader = parse_fastx_file(&filename).expect("valid path/file");
    let k_size=13;
    let w_size =20;
    let mut internal_id=0;
    for record in reader.records().into_iter() {
        let mut record_minis=vec![];
        let mut record_seq:String =String::new();
        //retreive the current record
        //println!("It over records");
        let seq_rec = record.expect("invalid record");
        let sequence = seq_rec.seq();
        //let local_seq= std::str::from_utf8(sequence).expect("The genomic sequence should be utf8").to_string();
        //in the next lines we make sure that we have a proper header and store it as string
        let id = seq_rec.id().to_string().split(' ').collect::<Vec<_>>()[0].to_string();
        if let Some(gene_map) = coords.get(id.as_str()) {
            //println!("GM {:?}",gene_map.keys());
            for (gene_id, exon_coords) in gene_map {
                for exon_coord in exon_coords{
                    let exon_seq = &sequence[exon_coord.startpos as usize..exon_coord.endpos as usize];
                    record_seq.push_str(  std::str::from_utf8(exon_seq).unwrap());
                }
            }
        }
        let mut exon_minis=vec![];
        generate_sorted_fastq_new_version::get_canonical_kmer_minimizers_hashed(record_seq.as_bytes(),k_size,w_size,&mut exon_minis);
        record_minis.append( &mut exon_minis);
        println!("Record_seq {}: {}",id,record_seq);
        clustering::generate_initial_cluster_map(&record_minis,init_cluster_map,internal_id);
        let id_vec= vec![];
        clusters.insert(cl_id,id_vec);
        internal_id += 1;
    }
}

fn parse_gtf_and_collect_coords(gtf_path: Option<&str>, coords:&mut FxHashMap<String,FxHashMap<i32,Vec<Coord_obj>>>){

    //let new_coords=FxHashMap::default();
    let reader = gff::Reader::from_file(gtf_path.unwrap(),GFF3);
    let mut gene_id=0;
    let mut gene_init=String::new();
    for record in reader.expect("The reader should find records").records() {
        let mut rec = record.ok().expect("Error reading record.");
        //we have a new gene
        if rec.feature_type()=="gene"{
            //see if we are in a new chromosome/scaffold
            if !coords.contains_key(rec.seqname()){
                //we are in a new chromosome/scaffold
                //reset the gene_id
                gene_id = 0;
                //
                let sname= rec.seqname().to_string();
                coords.insert(sname,FxHashMap::default());
            }
            //get the gene map out of coords
            let gene_map: &mut FxHashMap<i32,Vec<Coord_obj>> = coords.get_mut(rec.seqname()).expect("We made sure that the key exists in the HashMap");
            //add an empty vector that will contain the exon coordinates for this gene
            //
            gene_id += 1;
        }

        else if rec.feature_type()=="exon"{
            //println!("{} {} {}",rec.seqname(),rec.feature_type(),gene_id);
            if let Some(gene_map) = coords.get_mut(rec.seqname()) {
                if let Some(coord_vec)=gene_map.get_mut(&gene_id){
                    //println!("{:?}",coord_vec);
                    coord_vec.push(Coord_obj{startpos: *rec.start(),endpos: *rec.end() });
                    //println!("{:?}",coord_vec);
                }
                else{
                    let mut coords_vec=vec![];
                    coords_vec.push(Coord_obj{startpos: *rec.start(),endpos: *rec.end() });
                    gene_map.insert(gene_id,coords_vec);
                }




            }

        }
        //println!("coords {:?}",coords);

    }
}


pub(crate) fn resolve_gtf(gtf_path: Option<&str>, fasta_path: Option<&str>,clusters: &mut FxHashMap<i32, Vec<i32>>, cluster_map: &mut FxHashMap<u64, Vec<i32>>) {
    println!("Resolving GFF file ");
    let mut coords=FxHashMap::default();//: HashMap<K, HashMap<i32, Vec<Coord_obj>, BuildHasherDefault<FxHasher>>, BuildHasherDefault<FxHasher>> = FxHashMap::default();
    parse_gtf_and_collect_coords(gtf_path, &mut coords);
    println!("First step done");
    parse_fasta_and_gen_clusters(fasta_path,coords, clusters, cluster_map);

    //detectOverlaps(coords);
    //for coord in &coords{
    //    println!("id: {}",coord.0);
    //    for coord_e in coord.1{
    //        println!(" {}", coord_e)
    //    }
    //}
    println!("GTF resolved");
}


