using System;
using System.Net;
using System.Net.Http;
using System.Net.Http.Headers;
using System.Threading.Tasks;
using Newtonsoft.Json.Linq;

namespace net
{
    public class Product
    {
        public string Id { get; set; }
        public string Name { get; set; }
        public decimal Price { get; set; }
        public string Category { get; set; }
    }

    class Program
    {
        static HttpClient client = new HttpClient();
        static void Main(string[] args)
        {
            Task.Factory.StartNew(async () =>
            {
                Console.WriteLine("veamos...");
                //Product product = null;
                string path = "https://ws.smn.gob.ar/map_items/forecast/1";
                HttpResponseMessage response = await client.GetAsync(path);
                if (response.IsSuccessStatusCode)
                {
                    //product = await response.Content.ReadAsAsync<Product>();
                    var json = await response.Content.ReadAsStringAsync();
                    Console.WriteLine(json.GetType());

                    Console.WriteLine("json readed");

                    JObject report = JObject.Parse(json.ToString());

                    Console.WriteLine("REPORT CREATED");

                    Console.WriteLine("REPORT: {0}", report);

                    //var report0 = report[0];//["channel"]["title"];
                    //Console.WriteLine(report0);

                    foreach (var child in report.Children())
                    {
                        Console.WriteLine(child);
                    }
                }
                else
                {
                    Console.WriteLine("nada :(");
                }
                //return product;

            });

            Console.ReadKey(true);
        }
    }
}
